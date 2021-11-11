#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisement2 {
    type Vtable = IWiFiDirectAdvertisement2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb759aa46_d816_491b_917a_b40d7dc403a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectError) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i16) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParameters2 {
    type Vtable = IWiFiDirectConnectionParameters2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3b0fbe_aa82_44b4_88c8_e3056b89801d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectPairingProcedure) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: WiFiDirectPairingProcedure) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionParametersStatics {
    type Vtable = IWiFiDirectConnectionParametersStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x598af493_7642_456f_b9d8_e8a9eb1f401a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectDevice(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectConnectionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectDeviceStatics {
    type Vtable = IWiFiDirectDeviceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe86cb57c_3aac_4851_a792_482aaf931b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectDeviceStatics2 {
    type Vtable = IWiFiDirectDeviceStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a953e49_b103_437e_9226_ab67971342f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectInformationElementStatics {
    type Vtable = IWiFiDirectInformationElementStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbd02f16_11a5_4e60_8caa_34772148378a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectAdvertisement(pub ::windows::core::IInspectable);
impl WiFiDirectAdvertisement {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn InformationElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn SetInformationElements<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn ListenStateDiscoverability(&self) -> ::windows::core::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementListenStateDiscoverability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementListenStateDiscoverability>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn LegacySettings(&self) -> ::windows::core::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectLegacySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisement;{ab511a2d-2a06-49a1-a584-61435c7905a6})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectAdvertisement> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisement> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisement {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisement {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(0i32);
    pub const Normal: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(1i32);
    pub const Intensive: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(2i32);
}
impl ::core::convert::From<i32> for WiFiDirectAdvertisementListenStateDiscoverability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectAdvertisementListenStateDiscoverability {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectAdvertisementListenStateDiscoverability {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectAdvertisementPublisher(pub ::windows::core::IInspectable);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectAdvertisementPublisher, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Advertisement(&self) -> ::windows::core::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher;{b35a2d1a-9b1f-45d9-925a-694d66df68ef})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisher {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(0i32);
    pub const Started: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(1i32);
    pub const Stopped: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(2i32);
    pub const Aborted: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(3i32);
}
impl ::core::convert::From<i32> for WiFiDirectAdvertisementPublisherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectAdvertisementPublisherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectAdvertisementPublisherStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Error(&self) -> ::windows::core::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectError>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs;{aafde53c-5481-46e6-90dd-32116518f192})");
}
unsafe impl ::windows::core::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
impl ::windows::core::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(0i32);
    pub const DisplayPin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(1i32);
    pub const PushButton: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(2i32);
}
impl ::core::convert::From<i32> for WiFiDirectConfigurationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectConfigurationMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectConfigurationMethod {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectConnectionListener(pub ::windows::core::IInspectable);
impl WiFiDirectConnectionListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionListener, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn ConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionListener;{699c1b0d-8d13-4ee9-b9ec-9c72f8251f7d})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectConnectionListener> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionListener> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionListener {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionListener {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectConnectionParameters(pub ::windows::core::IInspectable);
impl WiFiDirectConnectionParameters {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionParameters, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GroupOwnerIntent(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetGroupOwnerIntent(&self, value: i16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn PreferredPairingProcedure(&self) -> ::windows::core::Result<WiFiDirectPairingProcedure> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: WiFiDirectPairingProcedure = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectPairingProcedure>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`*"]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::core::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__: super::Enumeration::DevicePairingKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configurationmethod, &mut result__).from_abi::<super::Enumeration::DevicePairingKinds>(result__)
        })
    }
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters;{b2e55405-5702-4b16-a02c-bbcd21ef6098})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionParameters> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectConnectionRequest(pub ::windows::core::IInspectable);
impl WiFiDirectConnectionRequest {
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest;{8eb99605-914f-49c3-a614-d18dc5b19b43})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequest> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequest> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectConnectionRequestedEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectConnectionRequestedEventArgs {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetConnectionRequest(&self) -> ::windows::core::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs;{f99d20be-d38d-484f-8215-e7b65abf244c})");
}
unsafe impl ::windows::core::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
impl ::windows::core::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequestedEventArgs {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(0i32);
    pub const Connected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(1i32);
}
impl ::core::convert::From<i32> for WiFiDirectConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectConnectionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectConnectionStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectDevice(pub ::windows::core::IInspectable);
impl WiFiDirectDevice {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectConnectionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`, `Networking`*"]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn FromIdAsync2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, WiFiDirectConnectionParameters>>(deviceid: Param0, connectionparameters: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), connectionparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectDevice;{72deaaa8-72eb-4dae-8a28-8513355d2777})");
}
unsafe impl ::windows::core::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
impl ::windows::core::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectDevice> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectDevice> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(0i32);
    pub const AssociationEndpoint: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(1i32);
}
impl ::core::convert::From<i32> for WiFiDirectDeviceSelectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectDeviceSelectorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectDeviceSelectorType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: WiFiDirectError = WiFiDirectError(0i32);
    pub const RadioNotAvailable: WiFiDirectError = WiFiDirectError(1i32);
    pub const ResourceInUse: WiFiDirectError = WiFiDirectError(2i32);
}
impl ::core::convert::From<i32> for WiFiDirectError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectError {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectInformationElement(pub ::windows::core::IInspectable);
impl WiFiDirectInformationElement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectInformationElement, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn Oui(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn SetOui<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn OuiType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetOuiType(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateFromDeviceInformation<'a, Param0: ::windows::core::IntoParam<'a, super::Enumeration::DeviceInformation>>(deviceinformation: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectInformationElement;{affb72d6-76bb-497e-ac8b-dc72838bc309})");
}
unsafe impl ::windows::core::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
impl ::windows::core::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectInformationElement) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectInformationElement> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectInformationElement) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectInformationElement> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectInformationElement {}
unsafe impl ::core::marker::Sync for WiFiDirectInformationElement {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectLegacySettings(pub ::windows::core::IInspectable);
impl WiFiDirectLegacySettings {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetSsid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Security_Credentials`*"]
    pub fn Passphrase(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Security_Credentials`*"]
    pub fn SetPassphrase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectLegacySettings;{a64fdbba-f2fd-4567-a91b-f5c2f5321057})");
}
unsafe impl ::windows::core::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
impl ::windows::core::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectLegacySettings> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectLegacySettings> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectLegacySettings {}
unsafe impl ::core::marker::Sync for WiFiDirectLegacySettings {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(0i32);
    pub const Invitation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(1i32);
}
impl ::core::convert::From<i32> for WiFiDirectPairingProcedure {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectPairingProcedure {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectPairingProcedure {
    type DefaultType = Self;
}
