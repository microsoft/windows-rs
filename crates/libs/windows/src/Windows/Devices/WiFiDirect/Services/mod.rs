#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectService {
    type Vtable = IWiFiDirectService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50aabbb8_5f71_45ec_84f1_a1e4fc7879a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectService_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub RemoteServiceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RemoteServiceInfo: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSessionInfo: usize,
    pub ServiceError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SessionDeferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionDeferred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionDeferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionDeferred: usize,
    #[cfg(feature = "Foundation")]
    pub GetProvisioningInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProvisioningInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsyncWithPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsyncWithPin: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAdvertiser {
    type Vtable = IWiFiDirectServiceAdvertiser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4aa1ee1_9d8f_4f4f_93ee_7ddea2e37f46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServiceNamePrefixes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServiceNamePrefixes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ServiceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ServiceInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetServiceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetServiceInfo: usize,
    pub AutoAcceptSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoAcceptSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PreferGroupOwnerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPreferGroupOwnerMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferredConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferredConfigurationMethods: usize,
    pub ServiceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceStatus) -> ::windows::core::HRESULT,
    pub SetServiceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectServiceStatus) -> ::windows::core::HRESULT,
    pub CustomServiceStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomServiceStatusCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDeferredSessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDeferredSessionInfo: usize,
    pub AdvertisementStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceAdvertisementStatus) -> ::windows::core::HRESULT,
    pub ServiceError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SessionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AutoAcceptSessionConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoAcceptSessionConnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoAcceptSessionConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoAcceptSessionConnected: usize,
    #[cfg(feature = "Foundation")]
    pub AdvertisementStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvertisementStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvertisementStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvertisementStatusChanged: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    ConnectAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub ConnectAsyncWithPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinfo: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    ConnectAsyncWithPin: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceAdvertiserFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAdvertiserFactory {
    type Vtable = IWiFiDirectServiceAdvertiserFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3106ac0d_b446_4f13_9f9a_8ae925feba2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAdvertiserFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateWiFiDirectServiceAdvertiser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    type Vtable = IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd9e01e_83df_43e5_8f43_cbe8479e84eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceProvisioningInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceProvisioningInfo {
    type Vtable = IWiFiDirectServiceProvisioningInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bdb7cfe_97d9_45a2_8e99_db50910fb6a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceProvisioningInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SelectedConfigurationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceConfigurationMethod) -> ::windows::core::HRESULT,
    pub IsGroupFormationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceRemotePortAddedEventArgs {
    type Vtable = IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cebac1_3fd3_4f0e_b7bd_782906f44411);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub EndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    EndpointPairs: usize,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceIPProtocol) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSession {
    type Vtable = IWiFiDirectServiceSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81142163_e426_47cb_8640_e1b3588bf26f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceSessionStatus) -> ::windows::core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceSessionErrorStatus) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AdvertisementId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ServiceAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SessionAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
    #[cfg(feature = "Foundation")]
    pub SessionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionStatusChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub AddStreamSocketListenerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    AddStreamSocketListenerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub AddDatagramSocketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    AddDatagramSocketAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemotePortAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemotePortAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemotePortAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemotePortAdded: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionDeferredEventArgs {
    type Vtable = IWiFiDirectServiceSessionDeferredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dfc197f_1201_4f1f_b6f4_5df1b7b9fb2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionDeferredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DeferredSessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeferredSessionInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionRequest {
    type Vtable = IWiFiDirectServiceSessionRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0e27c8b_50cb_4a58_9bcf_e472b99fba04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
    pub ProvisioningInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SessionInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceSessionRequestedEventArgs {
    type Vtable = IWiFiDirectServiceSessionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74bdcc11_53d6_4999_b4f8_6c8ecc1771e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceSessionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetSessionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectServiceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiDirectServiceStatics {
    type Vtable = IWiFiDirectServiceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7db40045_fd74_4688_b725_5dce86acf233);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectServiceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetSelectorWithFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceinfofilter: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetSelectorWithFilter: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectService(::windows::core::IUnknown);
impl WiFiDirectService {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RemoteServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteServiceInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedConfigurationMethods)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferGroupOwnerMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferGroupOwnerMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSessionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSessionInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceError>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionDeferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionDeferred)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionDeferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionDeferred)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetProvisioningInfoAsync(&self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProvisioningInfoAsync)(::core::mem::transmute_copy(this), selectedconfigurationmethod, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsyncWithPin<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pin: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsyncWithPin)(::core::mem::transmute_copy(this), pin.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn GetSelector<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(servicename: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSelector)(::core::mem::transmute_copy(this), servicename.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetSelectorWithFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(servicename: Param0, serviceinfofilter: Param1) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSelectorWithFilter)(::core::mem::transmute_copy(this), servicename.into_param().abi(), serviceinfofilter.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>> {
        Self::IWiFiDirectServiceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectServiceStatics<R, F: FnOnce(&IWiFiDirectServiceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectService, IWiFiDirectServiceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectService {}
impl ::core::fmt::Debug for WiFiDirectService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectService;{50aabbb8-5f71-45ec-84f1-a1e4fc7879a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectService {
    type Vtable = IWiFiDirectService_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectService as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectService {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectService";
}
impl ::core::convert::From<WiFiDirectService> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectService> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectService> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectService> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectService {}
unsafe impl ::core::marker::Sync for WiFiDirectService {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceAdvertisementStatus(pub i32);
impl WiFiDirectServiceAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceAdvertisementStatus {}
impl ::core::clone::Clone for WiFiDirectServiceAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceAdvertisementStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceAdvertisementStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceAdvertisementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceAdvertisementStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAdvertisementStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertisementStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceAdvertiser(::windows::core::IUnknown);
impl WiFiDirectServiceAdvertiser {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServiceNamePrefixes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceNamePrefixes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetServiceInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServiceInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn AutoAcceptSession(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoAcceptSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SetAutoAcceptSession(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoAcceptSession)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferGroupOwnerMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferGroupOwnerMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferredConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreferredConfigurationMethods)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceStatus(&self) -> ::windows::core::Result<WiFiDirectServiceStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SetServiceStatus(&self, value: WiFiDirectServiceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServiceStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn CustomServiceStatusCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomServiceStatusCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SetCustomServiceStatusCode(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomServiceStatusCode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeferredSessionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDeferredSessionInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeferredSessionInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn AdvertisementStatus(&self) -> ::windows::core::Result<WiFiDirectServiceAdvertisementStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceAdvertisementStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdvertisementStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceAdvertisementStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceError>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoAcceptSessionConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoAcceptSessionConnected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoAcceptSessionConnected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAutoAcceptSessionConnected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdvertisementStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdvertisementStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdvertisementStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn ConnectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Enumeration::DeviceInformation>>(&self, deviceinfo: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::core::mem::transmute_copy(this), deviceinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn ConnectAsyncWithPin<'a, Param0: ::windows::core::IntoParam<'a, super::super::Enumeration::DeviceInformation>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, deviceinfo: Param0, pin: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConnectAsyncWithPin)(::core::mem::transmute_copy(this), deviceinfo.into_param().abi(), pin.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn CreateWiFiDirectServiceAdvertiser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(servicename: Param0) -> ::windows::core::Result<WiFiDirectServiceAdvertiser> {
        Self::IWiFiDirectServiceAdvertiserFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWiFiDirectServiceAdvertiser)(::core::mem::transmute_copy(this), servicename.into_param().abi(), &mut result__).from_abi::<WiFiDirectServiceAdvertiser>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectServiceAdvertiserFactory<R, F: FnOnce(&IWiFiDirectServiceAdvertiserFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WiFiDirectServiceAdvertiser, IWiFiDirectServiceAdvertiserFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceAdvertiser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceAdvertiser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceAdvertiser {}
impl ::core::fmt::Debug for WiFiDirectServiceAdvertiser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceAdvertiser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAdvertiser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser;{a4aa1ee1-9d8f-4f4f-93ee-7ddea2e37f46})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceAdvertiser {
    type Vtable = IWiFiDirectServiceAdvertiser_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceAdvertiser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceAdvertiser {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAdvertiser";
}
impl ::core::convert::From<WiFiDirectServiceAdvertiser> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceAdvertiser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceAdvertiser> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceAdvertiser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceAdvertiser> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceAdvertiser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceAdvertiser> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceAdvertiser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceAdvertiser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceAdvertiser {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceAdvertiser {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceAutoAcceptSessionConnectedEventArgs(::windows::core::IUnknown);
impl WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn Session(&self) -> ::windows::core::Result<WiFiDirectServiceSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSession>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceAutoAcceptSessionConnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs;{dcd9e01e-83df-43e5-8f43-cbe8479e84eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    type Vtable = IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceAutoAcceptSessionConnectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceAutoAcceptSessionConnectedEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceConfigurationMethod(pub i32);
impl WiFiDirectServiceConfigurationMethod {
    pub const Default: Self = Self(0i32);
    pub const PinDisplay: Self = Self(1i32);
    pub const PinEntry: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceConfigurationMethod {}
impl ::core::clone::Clone for WiFiDirectServiceConfigurationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceConfigurationMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceConfigurationMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceConfigurationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceConfigurationMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceConfigurationMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceConfigurationMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceError(pub i32);
impl WiFiDirectServiceError {
    pub const Success: Self = Self(0i32);
    pub const RadioNotAvailable: Self = Self(1i32);
    pub const ResourceInUse: Self = Self(2i32);
    pub const UnsupportedHardware: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
}
impl ::core::marker::Copy for WiFiDirectServiceError {}
impl ::core::clone::Clone for WiFiDirectServiceError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceError {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceIPProtocol(pub i32);
impl WiFiDirectServiceIPProtocol {
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
}
impl ::core::marker::Copy for WiFiDirectServiceIPProtocol {}
impl ::core::clone::Clone for WiFiDirectServiceIPProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceIPProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceIPProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceIPProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceIPProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceIPProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceIPProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceProvisioningInfo(::windows::core::IUnknown);
impl WiFiDirectServiceProvisioningInfo {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SelectedConfigurationMethod(&self) -> ::windows::core::Result<WiFiDirectServiceConfigurationMethod> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceConfigurationMethod = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectedConfigurationMethod)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceConfigurationMethod>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn IsGroupFormationNeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGroupFormationNeeded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceProvisioningInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceProvisioningInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceProvisioningInfo {}
impl ::core::fmt::Debug for WiFiDirectServiceProvisioningInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceProvisioningInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceProvisioningInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo;{8bdb7cfe-97d9-45a2-8e99-db50910fb6a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceProvisioningInfo {
    type Vtable = IWiFiDirectServiceProvisioningInfo_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceProvisioningInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceProvisioningInfo {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceProvisioningInfo";
}
impl ::core::convert::From<WiFiDirectServiceProvisioningInfo> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceProvisioningInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceProvisioningInfo> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceProvisioningInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceProvisioningInfo> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceProvisioningInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceProvisioningInfo> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceProvisioningInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceProvisioningInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceProvisioningInfo {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceProvisioningInfo {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceRemotePortAddedEventArgs(::windows::core::IUnknown);
impl WiFiDirectServiceRemotePortAddedEventArgs {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation_Collections\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn EndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EndpointPairs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn Protocol(&self) -> ::windows::core::Result<WiFiDirectServiceIPProtocol> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceIPProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Protocol)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceIPProtocol>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceRemotePortAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceRemotePortAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceRemotePortAddedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectServiceRemotePortAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceRemotePortAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceRemotePortAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs;{d4cebac1-3fd3-4f0e-b7bd-782906f44411})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceRemotePortAddedEventArgs {
    type Vtable = IWiFiDirectServiceRemotePortAddedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceRemotePortAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceRemotePortAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceRemotePortAddedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceRemotePortAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceRemotePortAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceRemotePortAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceRemotePortAddedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceRemotePortAddedEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSession(::windows::core::IUnknown);
impl WiFiDirectServiceSession {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<WiFiDirectServiceSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ErrorStatus(&self) -> ::windows::core::Result<WiFiDirectServiceSessionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: WiFiDirectServiceSessionErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionErrorStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SessionId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn AdvertisementId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdvertisementId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ServiceAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn SessionAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation_Collections\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConnectionEndpointPairs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn AddStreamSocketListenerAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::Sockets::StreamSocketListener>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddStreamSocketListenerAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn AddDatagramSocketAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Networking::Sockets::DatagramSocket>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddDatagramSocketAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemotePortAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemotePortAdded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemotePortAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemotePortAdded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceSession {}
impl ::core::fmt::Debug for WiFiDirectServiceSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession;{81142163-e426-47cb-8640-e1b3588bf26f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSession {
    type Vtable = IWiFiDirectServiceSession_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSession {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSession";
}
impl ::core::convert::From<WiFiDirectServiceSession> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSession> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceSession> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSession> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionDeferredEventArgs(::windows::core::IUnknown);
impl WiFiDirectServiceSessionDeferredEventArgs {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeferredSessionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceSessionDeferredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceSessionDeferredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceSessionDeferredEventArgs {}
impl ::core::fmt::Debug for WiFiDirectServiceSessionDeferredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionDeferredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionDeferredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs;{8dfc197f-1201-4f1f-b6f4-5df1b7b9fb2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionDeferredEventArgs {
    type Vtable = IWiFiDirectServiceSessionDeferredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceSessionDeferredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionDeferredEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionDeferredEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionDeferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionDeferredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionDeferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSessionDeferredEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSessionDeferredEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceSessionErrorStatus(pub i32);
impl WiFiDirectServiceSessionErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Disassociated: Self = Self(1i32);
    pub const LocalClose: Self = Self(2i32);
    pub const RemoteClose: Self = Self(3i32);
    pub const SystemFailure: Self = Self(4i32);
    pub const NoResponseFromRemote: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionErrorStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceSessionErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceSessionErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceSessionErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionErrorStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequest(::windows::core::IUnknown);
impl WiFiDirectServiceSessionRequest {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Enumeration::DeviceInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn ProvisioningInfo(&self) -> ::windows::core::Result<WiFiDirectServiceProvisioningInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProvisioningInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceProvisioningInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceSessionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceSessionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceSessionRequest {}
impl ::core::fmt::Debug for WiFiDirectServiceSessionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest;{a0e27c8b-50cb-4a58-9bcf-e472b99fba04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionRequest {
    type Vtable = IWiFiDirectServiceSessionRequest_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceSessionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequest";
}
impl ::core::convert::From<WiFiDirectServiceSessionRequest> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequest> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionRequest> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequest> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
pub struct WiFiDirectServiceSessionRequestedEventArgs(::windows::core::IUnknown);
impl WiFiDirectServiceSessionRequestedEventArgs {
    #[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
    pub fn GetSessionRequest(&self) -> ::windows::core::Result<WiFiDirectServiceSessionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSessionRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WiFiDirectServiceSessionRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiDirectServiceSessionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiDirectServiceSessionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiDirectServiceSessionRequestedEventArgs {}
impl ::core::fmt::Debug for WiFiDirectServiceSessionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs;{74bdcc11-53d6-4999-b4f8-6c8ecc1771e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WiFiDirectServiceSessionRequestedEventArgs {
    type Vtable = IWiFiDirectServiceSessionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWiFiDirectServiceSessionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WiFiDirectServiceSessionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionRequestedEventArgs";
}
impl ::core::convert::From<WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiDirectServiceSessionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WiFiDirectServiceSessionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WiFiDirectServiceSessionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiDirectServiceSessionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectServiceSessionRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceSessionStatus(pub i32);
impl WiFiDirectServiceSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Initiated: Self = Self(1i32);
    pub const Requested: Self = Self(2i32);
    pub const Open: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiDirectServiceSessionStatus {}
impl ::core::clone::Clone for WiFiDirectServiceSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceSessionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceSessionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceSessionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceSessionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceSessionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_WiFiDirect_Services\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiDirectServiceStatus(pub i32);
impl WiFiDirectServiceStatus {
    pub const Available: Self = Self(0i32);
    pub const Busy: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiDirectServiceStatus {}
impl ::core::clone::Clone for WiFiDirectServiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiDirectServiceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WiFiDirectServiceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiDirectServiceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectServiceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WiFiDirectServiceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.Services.WiFiDirectServiceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
