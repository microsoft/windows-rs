#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2874219053, 10758, 18849, [165, 132, 97, 67, 92, 121, 5, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectAdvertisement2 {
    type Vtable = IWiFiDirectAdvertisement2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3076106822, 55318, 18715, [145, 122, 180, 13, 125, 196, 3, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3009031450, 39711, 17881, [146, 90, 105, 77, 102, 223, 104, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2868766012, 21633, 18150, [144, 221, 50, 17, 101, 24, 241, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectError) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771838221, 36115, 20201, [185, 236, 156, 114, 248, 37, 31, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3001373701, 22274, 19222, [160, 44, 187, 205, 33, 239, 96, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionParameters2 {
    type Vtable = IWiFiDirectConnectionParameters2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2872774590, 43650, 17588, [136, 200, 227, 5, 107, 137, 128, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectPairingProcedure) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: WiFiDirectPairingProcedure) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionParametersStatics {
    type Vtable = IWiFiDirectConnectionParametersStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1502278803, 30274, 17775, [185, 216, 232, 169, 235, 31, 64, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2394527237, 37199, 18883, [166, 20, 209, 141, 197, 177, 155, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4187824318, 54157, 18511, [130, 21, 231, 182, 90, 191, 36, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1927195304, 29419, 19886, [138, 40, 133, 19, 53, 93, 39, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WiFiDirectConnectionStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectDeviceStatics {
    type Vtable = IWiFiDirectDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3899438460, 15020, 18513, [167, 146, 72, 42, 175, 147, 27, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectDeviceStatics2 {
    type Vtable = IWiFiDirectDeviceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(445988425, 45315, 17278, [146, 38, 171, 103, 151, 19, 66, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, connectionparameters: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2952491734, 30395, 18814, [172, 139, 220, 114, 131, 139, 195, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectInformationElementStatics {
    type Vtable = IWiFiDirectInformationElementStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3687853846, 4517, 20064, [140, 170, 52, 119, 33, 72, 55, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceinformation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2790251450, 62205, 17767, [169, 27, 245, 194, 245, 50, 16, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectAdvertisement(::windows::runtime::IInspectable);
impl WiFiDirectAdvertisement {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn InformationElements(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn SetInformationElements<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn ListenStateDiscoverability(&self) -> ::windows::runtime::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementListenStateDiscoverability = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementListenStateDiscoverability>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn LegacySettings(&self) -> ::windows::runtime::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectLegacySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn SupportedConfigurationMethods(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::runtime::Interface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisement;{ab511a2d-2a06-49a1-a584-61435c7905a6})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2874219053, 10758, 18849, [165, 132, 97, 67, 92, 121, 5, 166]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
impl ::std::convert::From<WiFiDirectAdvertisement> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisement> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectAdvertisement> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectAdvertisement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisement> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectAdvertisement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectAdvertisement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectAdvertisement {}
unsafe impl ::std::marker::Sync for WiFiDirectAdvertisement {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementListenStateDiscoverability(pub i32);
impl WiFiDirectAdvertisementListenStateDiscoverability {
    pub const None: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(0i32);
    pub const Normal: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(1i32);
    pub const Intensive: WiFiDirectAdvertisementListenStateDiscoverability = WiFiDirectAdvertisementListenStateDiscoverability(2i32);
}
impl ::std::convert::From<i32> for WiFiDirectAdvertisementListenStateDiscoverability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectAdvertisementListenStateDiscoverability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectAdvertisementPublisher(::windows::runtime::IInspectable);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectAdvertisementPublisher, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Advertisement(&self) -> ::windows::runtime::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisement>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher;{b35a2d1a-9b1f-45d9-925a-694d66df68ef})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3009031450, 39711, 17881, [146, 90, 105, 77, 102, 223, 104, 239]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
impl ::std::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectAdvertisementPublisher> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisementPublisher> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectAdvertisementPublisher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectAdvertisementPublisher {}
unsafe impl ::std::marker::Sync for WiFiDirectAdvertisementPublisher {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatus(pub i32);
impl WiFiDirectAdvertisementPublisherStatus {
    pub const Created: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(0i32);
    pub const Started: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(1i32);
    pub const Stopped: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(2i32);
    pub const Aborted: WiFiDirectAdvertisementPublisherStatus = WiFiDirectAdvertisementPublisherStatus(3i32);
}
impl ::std::convert::From<i32> for WiFiDirectAdvertisementPublisherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectAdvertisementPublisherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows::runtime::IInspectable);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectAdvertisementPublisherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectAdvertisementPublisherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Error(&self) -> ::windows::runtime::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectError = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs;{aafde53c-5481-46e6-90dd-32116518f192})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2868766012, 21633, 18150, [144, 221, 50, 17, 101, 24, 241, 146]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
impl ::std::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectAdvertisementPublisherStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectAdvertisementPublisherStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::std::marker::Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectConfigurationMethod(pub i32);
impl WiFiDirectConfigurationMethod {
    pub const ProvidePin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(0i32);
    pub const DisplayPin: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(1i32);
    pub const PushButton: WiFiDirectConfigurationMethod = WiFiDirectConfigurationMethod(2i32);
}
impl ::std::convert::From<i32> for WiFiDirectConfigurationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectConfigurationMethod {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectConnectionListener(::windows::runtime::IInspectable);
impl WiFiDirectConnectionListener {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectConnectionListener, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn ConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveConnectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionListener;{699c1b0d-8d13-4ee9-b9ec-9c72f8251f7d})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1771838221, 36115, 20201, [185, 236, 156, 114, 248, 37, 31, 125]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
impl ::std::convert::From<WiFiDirectConnectionListener> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectConnectionListener> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectConnectionListener> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectConnectionListener) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectConnectionListener> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectConnectionListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectConnectionListener {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectConnectionListener {}
unsafe impl ::std::marker::Sync for WiFiDirectConnectionListener {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectConnectionParameters(::windows::runtime::IInspectable);
impl WiFiDirectConnectionParameters {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectConnectionParameters, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GroupOwnerIntent(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetGroupOwnerIntent(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`*"]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows::runtime::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn PreferredPairingProcedure(&self) -> ::windows::runtime::Result<WiFiDirectPairingProcedure> {
        let this = &::windows::runtime::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__: WiFiDirectPairingProcedure = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectPairingProcedure>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`*"]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> ::windows::runtime::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__: super::Enumeration::DevicePairingKinds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), configurationmethod, &mut result__).from_abi::<super::Enumeration::DevicePairingKinds>(result__)
        })
    }
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters;{b2e55405-5702-4b16-a02c-bbcd21ef6098})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3001373701, 22274, 19222, [160, 44, 187, 205, 33, 239, 96, 152]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
impl ::std::convert::From<WiFiDirectConnectionParameters> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectConnectionParameters> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectConnectionParameters> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectConnectionParameters) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectConnectionParameters> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectConnectionParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::std::convert::TryFrom<WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WiFiDirectConnectionParameters) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl ::std::convert::TryFrom<&WiFiDirectConnectionParameters> for super::Enumeration::IDevicePairingSettings {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WiFiDirectConnectionParameters) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Devices_Enumeration")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Enumeration::IDevicePairingSettings> for &WiFiDirectConnectionParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Enumeration::IDevicePairingSettings> {
        ::std::convert::TryInto::<super::Enumeration::IDevicePairingSettings>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectConnectionParameters {}
unsafe impl ::std::marker::Sync for WiFiDirectConnectionParameters {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectConnectionRequest(::windows::runtime::IInspectable);
impl WiFiDirectConnectionRequest {
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest;{8eb99605-914f-49c3-a614-d18dc5b19b43})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2394527237, 37199, 18883, [166, 20, 209, 141, 197, 177, 155, 67]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
impl ::std::convert::From<WiFiDirectConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectConnectionRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectConnectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WiFiDirectConnectionRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&WiFiDirectConnectionRequest> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WiFiDirectConnectionRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectConnectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectConnectionRequest {}
unsafe impl ::std::marker::Sync for WiFiDirectConnectionRequest {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectConnectionRequestedEventArgs(::windows::runtime::IInspectable);
impl WiFiDirectConnectionRequestedEventArgs {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetConnectionRequest(&self) -> ::windows::runtime::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs;{f99d20be-d38d-484f-8215-e7b65abf244c})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4187824318, 54157, 18511, [130, 21, 231, 182, 90, 191, 36, 76]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
impl ::std::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectConnectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectConnectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectConnectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl ::std::marker::Sync for WiFiDirectConnectionRequestedEventArgs {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectConnectionStatus(pub i32);
impl WiFiDirectConnectionStatus {
    pub const Disconnected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(0i32);
    pub const Connected: WiFiDirectConnectionStatus = WiFiDirectConnectionStatus(1i32);
}
impl ::std::convert::From<i32> for WiFiDirectConnectionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectConnectionStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectDevice(::windows::runtime::IInspectable);
impl WiFiDirectDevice {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn ConnectionStatus(&self) -> ::windows::runtime::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectConnectionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectConnectionStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn ConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn RemoveConnectionStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`, `Networking`*"]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation`*"]
    pub fn FromIdAsync2<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, WiFiDirectConnectionParameters>>(deviceid: Param0, connectionparameters: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), connectionparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>(result__)
        })
    }
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectDevice;{72deaaa8-72eb-4dae-8a28-8513355d2777})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1927195304, 29419, 19886, [138, 40, 133, 19, 53, 93, 39, 119]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
impl ::std::convert::From<WiFiDirectDevice> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectDevice> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectDevice> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectDevice> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WiFiDirectDevice) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&WiFiDirectDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WiFiDirectDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &WiFiDirectDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectDevice {}
unsafe impl ::std::marker::Sync for WiFiDirectDevice {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectDeviceSelectorType(pub i32);
impl WiFiDirectDeviceSelectorType {
    pub const DeviceInterface: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(0i32);
    pub const AssociationEndpoint: WiFiDirectDeviceSelectorType = WiFiDirectDeviceSelectorType(1i32);
}
impl ::std::convert::From<i32> for WiFiDirectDeviceSelectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectDeviceSelectorType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectError(pub i32);
impl WiFiDirectError {
    pub const Success: WiFiDirectError = WiFiDirectError(0i32);
    pub const RadioNotAvailable: WiFiDirectError = WiFiDirectError(1i32);
    pub const ResourceInUse: WiFiDirectError = WiFiDirectError(2i32);
}
impl ::std::convert::From<i32> for WiFiDirectError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectError {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectInformationElement(::windows::runtime::IInspectable);
impl WiFiDirectInformationElement {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectInformationElement, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn Oui(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn SetOui<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn OuiType(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetOuiType(&self, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Storage_Streams`*"]
    pub fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn CreateFromBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(buffer: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Devices_Enumeration`, `Foundation_Collections`*"]
    pub fn CreateFromDeviceInformation<'a, Param0: ::windows::runtime::IntoParam<'a, super::Enumeration::DeviceInformation>>(deviceinformation: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceinformation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>(result__)
        })
    }
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectInformationElement;{affb72d6-76bb-497e-ac8b-dc72838bc309})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2952491734, 30395, 18814, [172, 139, 220, 114, 131, 139, 195, 9]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
impl ::std::convert::From<WiFiDirectInformationElement> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectInformationElement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectInformationElement> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectInformationElement> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectInformationElement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectInformationElement> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectInformationElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectInformationElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectInformationElement {}
unsafe impl ::std::marker::Sync for WiFiDirectInformationElement {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WiFiDirectLegacySettings(::windows::runtime::IInspectable);
impl WiFiDirectLegacySettings {
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn Ssid(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_WiFiDirect`*"]
    pub fn SetSsid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Security_Credentials`*"]
    pub fn Passphrase(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Devices_WiFiDirect`, `Security_Credentials`*"]
    pub fn SetPassphrase<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectLegacySettings;{a64fdbba-f2fd-4567-a91b-f5c2f5321057})");
}
unsafe impl ::windows::runtime::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2790251450, 62205, 17767, [169, 27, 245, 194, 245, 50, 16, 87]);
}
impl ::windows::runtime::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
impl ::std::convert::From<WiFiDirectLegacySettings> for ::windows::runtime::IUnknown {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WiFiDirectLegacySettings> for ::windows::runtime::IUnknown {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WiFiDirectLegacySettings> for ::windows::runtime::IInspectable {
    fn from(value: WiFiDirectLegacySettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WiFiDirectLegacySettings> for ::windows::runtime::IInspectable {
    fn from(value: &WiFiDirectLegacySettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WiFiDirectLegacySettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WiFiDirectLegacySettings {}
unsafe impl ::std::marker::Sync for WiFiDirectLegacySettings {}
#[doc = "*Required features: `Devices_WiFiDirect`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectPairingProcedure(pub i32);
impl WiFiDirectPairingProcedure {
    pub const GroupOwnerNegotiation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(0i32);
    pub const Invitation: WiFiDirectPairingProcedure = WiFiDirectPairingProcedure(1i32);
}
impl ::std::convert::From<i32> for WiFiDirectPairingProcedure {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WiFiDirectPairingProcedure {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
}
