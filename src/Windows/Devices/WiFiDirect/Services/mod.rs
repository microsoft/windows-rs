#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectService(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectService {
    type Vtable = IWiFiDirectService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50aabbb8_5f71_45ec_84f1_a1e4fc7879a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAdvertiser {
    type Vtable = IWiFiDirectServiceAdvertiser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4aa1ee1_9d8f_4f4f_93ee_7ddea2e37f46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: WiFiDirectServiceStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceAdvertisementStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceinfo: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiserFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAdvertiserFactory {
    type Vtable = IWiFiDirectServiceAdvertiserFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3106ac0d_b446_4f13_9f9a_8ae925feba2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiserFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    type Vtable = IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd9e01e_83df_43e5_8f43_cbe8479e84eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceProvisioningInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceProvisioningInfo {
    type Vtable = IWiFiDirectServiceProvisioningInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdb7cfe_97d9_45a2_8e99_db50910fb6a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceProvisioningInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceConfigurationMethod) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceRemotePortAddedEventArgs {
    type Vtable = IWiFiDirectServiceRemotePortAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cebac1_3fd3_4f0e_b7bd_782906f44411);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceIPProtocol) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSession {
    type Vtable = IWiFiDirectServiceSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81142163_e426_47cb_8640_e1b3588bf26f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceSessionStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut WiFiDirectServiceSessionErrorStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionDeferredEventArgs {
    type Vtable = IWiFiDirectServiceSessionDeferredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dfc197f_1201_4f1f_b6f4_5df1b7b9fb2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionRequest {
    type Vtable = IWiFiDirectServiceSessionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e27c8b_50cb_4a58_9bcf_e472b99fba04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionRequestedEventArgs {
    type Vtable = IWiFiDirectServiceSessionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74bdcc11_53d6_4999_b4f8_6c8ecc1771e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs_abi(
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
pub struct IWiFiDirectServiceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceStatics {
    type Vtable = IWiFiDirectServiceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7db40045_fd74_4688_b725_5dce86acf233);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceinfofilter: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectService(pub ::windows::core::IInspectable);
impl WiFiDirectService {
    #[cfg(feature = "Storage_Streams")]
    pub fn RemoteServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>>(result__)
        }
    }
    pub fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSessionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SessionDeferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionDeferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetProvisioningInfoAsync(&self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), selectedconfigurationmethod, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsyncWithPin<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pin: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    pub fn GetSelector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(servicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), servicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetSelectorWithFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(servicename: Param0, serviceinfofilter: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), servicename.into_param().abi(), serviceinfofilter.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>>(result__)
        })
    }
    pub fn IWiFiDirectServiceStatics<R, F: FnOnce(&IWiFiDirectServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectService, IWiFiDirectServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectService;{50aabbb8-5f71-45ec-84f1-a1e4fc7879a3})");
}
unsafe impl ::windows::core::Interface for WiFiDirectService {
    type Vtable = IWiFiDirectService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50aabbb8_5f71_45ec_84f1_a1e4fc7879a3);
}
impl ::windows::core::RuntimeName for WiFiDirectService {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectService";
}
impl ::core::convert::From<WiFiDirectService> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectService) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectService> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectService) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectService> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectService> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectService {}
unsafe impl ::core::marker::Sync for WiFiDirectService {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(0i32);
    pub const Started: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(1i32);
    pub const Stopped: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(2i32);
    pub const Aborted: WiFiDirectServiceAdvertisementStatus = WiFiDirectServiceAdvertisementStatus(3i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceAdvertisementStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceAdvertisementStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAdvertisementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertisementStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceAdvertisementStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceAdvertiser(pub ::windows::core::IInspectable);
impl WiFiDirectServiceAdvertiser {
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceNamePrefixes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetServiceInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AutoAcceptSession(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoAcceptSession(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>>(result__)
        }
    }
    pub fn ServiceStatus(&self) -> ::windows::core::Result<WiFiDirectServiceStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceStatus>(result__)
        }
    }
    pub fn SetServiceStatus(&self, value: WiFiDirectServiceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CustomServiceStatusCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetCustomServiceStatusCode(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDeferredSessionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AdvertisementStatus(&self) -> ::windows::core::Result<WiFiDirectServiceAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceAdvertisementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceAdvertisementStatus>(result__)
        }
    }
    pub fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SessionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AutoAcceptSessionConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoAcceptSessionConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Enumeration::DeviceInformation>>(&self, deviceinfo: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), deviceinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn ConnectAsyncWithPin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Enumeration::DeviceInformation>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, deviceinfo: Param0, pin: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), deviceinfo.into_param().abi(), pin.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn CreateWiFiDirectServiceAdvertiser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(servicename: Param0) -> ::windows::core::Result<WiFiDirectServiceAdvertiser> {
        Self::IWiFiDirectServiceAdvertiserFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), servicename.into_param().abi(), &mut result__).from_abi::<WiFiDirectServiceAdvertiser>(result__)
        })
    }
    pub fn IWiFiDirectServiceAdvertiserFactory<R, F: FnOnce(&IWiFiDirectServiceAdvertiserFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectServiceAdvertiser, IWiFiDirectServiceAdvertiserFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAdvertiser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser;{a4aa1ee1-9d8f-4f4f-93ee-7ddea2e37f46})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceAdvertiser {
    type Vtable = IWiFiDirectServiceAdvertiser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4aa1ee1_9d8f_4f4f_93ee_7ddea2e37f46);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceAdvertiser {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser";
}
impl ::core::convert::From<WiFiDirectServiceAdvertiser> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceAdvertiser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceAdvertiser> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceAdvertiser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceAdvertiser> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceAdvertiser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceAdvertiser> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceAdvertiser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceAdvertiser {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceAdvertiser {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    pub fn Session(&self) -> ::windows::core::Result<WiFiDirectServiceSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSession>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs;{dcd9e01e-83df-43e5-8f43-cbe8479e84eb})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    type Vtable = IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd9e01e_83df_43e5_8f43_cbe8479e84eb);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(0i32);
    pub const PinDisplay: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(1i32);
    pub const PinEntry: WiFiDirectServiceConfigurationMethod = WiFiDirectServiceConfigurationMethod(2i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceConfigurationMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceConfigurationMethod {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceConfigurationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceConfigurationMethod {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: WiFiDirectServiceError = WiFiDirectServiceError(0i32);
    pub const RadioNotAvailable: WiFiDirectServiceError = WiFiDirectServiceError(1i32);
    pub const ResourceInUse: WiFiDirectServiceError = WiFiDirectServiceError(2i32);
    pub const UnsupportedHardware: WiFiDirectServiceError = WiFiDirectServiceError(3i32);
    pub const NoHardware: WiFiDirectServiceError = WiFiDirectServiceError(4i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceError {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceError;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceError {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: WiFiDirectServiceIPProtocol = WiFiDirectServiceIPProtocol(6i32);
    pub const Udp: WiFiDirectServiceIPProtocol = WiFiDirectServiceIPProtocol(17i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceIPProtocol {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceIPProtocol {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceIPProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceIPProtocol;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceIPProtocol {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceProvisioningInfo(pub ::windows::core::IInspectable);
impl WiFiDirectServiceProvisioningInfo {
    pub fn SelectedConfigurationMethod(&self) -> ::windows::core::Result<WiFiDirectServiceConfigurationMethod> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceConfigurationMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceConfigurationMethod>(result__)
        }
    }
    pub fn IsGroupFormationNeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceProvisioningInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo;{8bdb7cfe-97d9-45a2-8e99-db50910fb6a6})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceProvisioningInfo {
    type Vtable = IWiFiDirectServiceProvisioningInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdb7cfe_97d9_45a2_8e99_db50910fb6a6);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceProvisioningInfo {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo";
}
impl ::core::convert::From<WiFiDirectServiceProvisioningInfo> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceProvisioningInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceProvisioningInfo> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceProvisioningInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceProvisioningInfo> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceProvisioningInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceProvisioningInfo> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceProvisioningInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceProvisioningInfo {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceProvisioningInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectServiceRemotePortAddedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn EndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows::core::Result<WiFiDirectServiceIPProtocol> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceIPProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceIPProtocol>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceRemotePortAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs;{d4cebac1-3fd3-4f0e-b7bd-782906f44411})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceRemotePortAddedEventArgs {
    type Vtable = IWiFiDirectServiceRemotePortAddedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cebac1_3fd3_4f0e_b7bd_782906f44411);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceRemotePortAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceRemotePortAddedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceRemotePortAddedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceSession(pub ::windows::core::IInspectable);
impl WiFiDirectServiceSession {
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectServiceSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionStatus>(result__)
        }
    }
    pub fn ErrorStatus(&self) -> ::windows::core::Result<WiFiDirectServiceSessionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceSessionErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionErrorStatus>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AdvertisementId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ServiceAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SessionAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn AddStreamSocketListenerAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn AddDatagramSocketAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::Sockets::DatagramSocket>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemotePortAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemotePortAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession;{81142163-e426-47cb-8640-e1b3588bf26f})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSession {
    type Vtable = IWiFiDirectServiceSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81142163_e426_47cb_8640_e1b3588bf26f);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSession {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession";
}
impl ::core::convert::From<WiFiDirectServiceSession> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSession> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceSession> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSession> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectServiceSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: WiFiDirectServiceSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectServiceSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &WiFiDirectServiceSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSession {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectServiceSessionDeferredEventArgs {
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionDeferredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs;{8dfc197f-1201-4f1f-b6f4-5df1b7b9fb2e})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionDeferredEventArgs {
    type Vtable = IWiFiDirectServiceSessionDeferredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dfc197f_1201_4f1f_b6f4_5df1b7b9fb2e);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionDeferredEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSessionDeferredEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSessionDeferredEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(0i32);
    pub const Disassociated: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(1i32);
    pub const LocalClose: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(2i32);
    pub const RemoteClose: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(3i32);
    pub const SystemFailure: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(4i32);
    pub const NoResponseFromRemote: WiFiDirectServiceSessionErrorStatus = WiFiDirectServiceSessionErrorStatus(5i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceSessionErrorStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceSessionErrorStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionErrorStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceSessionErrorStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceSessionRequest(pub ::windows::core::IInspectable);
impl WiFiDirectServiceSessionRequest {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn ProvisioningInfo(&self) -> ::windows::core::Result<WiFiDirectServiceProvisioningInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceProvisioningInfo>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest;{a0e27c8b-50cb-4a58-9bcf-e472b99fba04})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionRequest {
    type Vtable = IWiFiDirectServiceSessionRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e27c8b_50cb_4a58_9bcf_e472b99fba04);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest";
}
impl ::core::convert::From<WiFiDirectServiceSessionRequest> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequest> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionRequest> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequest> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<WiFiDirectServiceSessionRequest> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: WiFiDirectServiceSessionRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&WiFiDirectServiceSessionRequest> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &WiFiDirectServiceSessionRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSessionRequest {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSessionRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(pub ::windows::core::IInspectable);
impl WiFiDirectServiceSessionRequestedEventArgs {
    pub fn GetSessionRequest(&self) -> ::windows::core::Result<WiFiDirectServiceSessionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs;{74bdcc11-53d6-4999-b4f8-6c8ecc1771e7})");
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionRequestedEventArgs {
    type Vtable = IWiFiDirectServiceSessionRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74bdcc11_53d6_4999_b4f8_6c8ecc1771e7);
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSessionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSessionRequestedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(0i32);
    pub const Initiated: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(1i32);
    pub const Requested: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(2i32);
    pub const Open: WiFiDirectServiceSessionStatus = WiFiDirectServiceSessionStatus(3i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceSessionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceSessionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceSessionStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: WiFiDirectServiceStatus = WiFiDirectServiceStatus(0i32);
    pub const Busy: WiFiDirectServiceStatus = WiFiDirectServiceStatus(1i32);
    pub const Custom: WiFiDirectServiceStatus = WiFiDirectServiceStatus(2i32);
}
impl ::core::convert::From<i32> for WiFiDirectServiceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceStatus;i4)");
}
impl ::windows::core::DefaultType for WiFiDirectServiceStatus {
    type DefaultType = Self;
}
