#[cfg(feature = "Devices_WiFiDirect_Services")]
pub mod Services;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectAdvertisement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab511a2d_2a06_49a1_a584_61435c7905a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub InformationElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InformationElements: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetInformationElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetInformationElements: usize,
    pub ListenStateDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::HRESULT,
    pub SetListenStateDiscoverability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::HRESULT,
    pub IsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAutonomousGroupOwnerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub LegacySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisement2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisement2 {
    type Vtable = IWiFiDirectAdvertisement2_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectAdvertisement2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectAdvertisement2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb759aa46_d816_491b_917a_b40d7dc403a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisement2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedConfigurationMethods: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectAdvertisementPublisher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35a2d1a_9b1f_45d9_925a_694d66df68ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaafde53c_5481_46e6_90dd_32116518f192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectAdvertisementPublisherStatus) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectError) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionListener(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x699c1b0d_8d13_4ee9_b9ec_9c72f8251f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionListener_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionParameters {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2e55405_5702_4b16_a02c_bbcd21ef6098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GroupOwnerIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT,
    pub SetGroupOwnerIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParameters2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParameters2 {
    type Vtable = IWiFiDirectConnectionParameters2_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionParameters2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionParameters2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab3b0fbe_aa82_44b4_88c8_e3056b89801d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParameters2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub PreferenceOrderedConfigurationMethods: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PreferenceOrderedConfigurationMethods: usize,
    pub PreferredPairingProcedure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectPairingProcedure) -> ::windows_core::HRESULT,
    pub SetPreferredPairingProcedure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiDirectPairingProcedure) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionParametersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionParametersStatics {
    type Vtable = IWiFiDirectConnectionParametersStatics_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionParametersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionParametersStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x598af493_7642_456f_b9d8_e8a9eb1f401a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionParametersStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub GetDevicePairingKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configurationmethod: WiFiDirectConfigurationMethod, result__: *mut super::Enumeration::DevicePairingKinds) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    GetDevicePairingKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8eb99605_914f_49c3_a614_d18dc5b19b43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectConnectionRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectConnectionRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf99d20be_d38d_484f_8215_e7b65abf244c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectConnectionRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetConnectionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72deaaa8_72eb_4dae_8a28_8513355d2777);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectConnectionStatus) -> ::windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetConnectionEndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetConnectionEndpointPairs: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDeviceStatics {
    type Vtable = IWiFiDirectDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe86cb57c_3aac_4851_a792_482aaf931b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectDeviceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectDeviceStatics2 {
    type Vtable = IWiFiDirectDeviceStatics2_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectDeviceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectDeviceStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a953e49_b103_437e_9226_ab67971342f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectDeviceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: WiFiDirectDeviceSelectorType, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, connectionparameters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectInformationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectInformationElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaffb72d6_76bb_497e_ac8b_dc72838bc309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Oui: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Oui: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetOui: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetOui: usize,
    pub OuiType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub SetOuiType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Value: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectInformationElementStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectInformationElementStatics {
    type Vtable = IWiFiDirectInformationElementStatics_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectInformationElementStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectInformationElementStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbd02f16_11a5_4e60_8caa_34772148378a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectInformationElementStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub CreateFromDeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinformation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections")))]
    CreateFromDeviceInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiDirectLegacySettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_Vtbl;
}
impl ::core::clone::Clone for IWiFiDirectLegacySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWiFiDirectLegacySettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa64fdbba_f2fd_4567_a91b_f5c2f5321057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiDirectLegacySettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub Passphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Passphrase: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPassphrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPassphrase: usize,
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisement(::windows_core::IUnknown);
impl WiFiDirectAdvertisement {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InformationElements(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InformationElements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetInformationElements<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInformationElements)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn ListenStateDiscoverability(&self) -> ::windows_core::Result<WiFiDirectAdvertisementListenStateDiscoverability> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ListenStateDiscoverability)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetListenStateDiscoverability(&self, value: WiFiDirectAdvertisementListenStateDiscoverability) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListenStateDiscoverability)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAutonomousGroupOwnerEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAutonomousGroupOwnerEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAutonomousGroupOwnerEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAutonomousGroupOwnerEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LegacySettings(&self) -> ::windows_core::Result<WiFiDirectLegacySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LegacySettings)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedConfigurationMethods(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows_core::ComInterface::cast::<IWiFiDirectAdvertisement2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedConfigurationMethods)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for WiFiDirectAdvertisement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisement;{ab511a2d-2a06-49a1-a584-61435c7905a6})");
}
impl ::core::clone::Clone for WiFiDirectAdvertisement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisement {
    type Vtable = IWiFiDirectAdvertisement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectAdvertisement {
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisement";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisement, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectAdvertisement {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisement {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisher(::windows_core::IUnknown);
impl WiFiDirectAdvertisementPublisher {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectAdvertisementPublisher, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Advertisement(&self) -> ::windows_core::Result<WiFiDirectAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WiFiDirectAdvertisementPublisher, WiFiDirectAdvertisementPublisherStatusChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher;{b35a2d1a-9b1f-45d9-925a-694d66df68ef})");
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisementPublisher {
    type Vtable = IWiFiDirectAdvertisementPublisher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectAdvertisementPublisher {
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisementPublisher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisher";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisementPublisher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisher {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisher {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectAdvertisementPublisherStatusChangedEventArgs(::windows_core::IUnknown);
impl WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<WiFiDirectAdvertisementPublisherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows_core::Result<WiFiDirectError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs;{aafde53c-5481-46e6-90dd-32116518f192})");
}
impl ::core::clone::Clone for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    type Vtable = IWiFiDirectAdvertisementPublisherStatusChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const IID: ::windows_core::GUID = <IWiFiDirectAdvertisementPublisherStatusChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatusChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectAdvertisementPublisherStatusChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectAdvertisementPublisherStatusChangedEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionListener(::windows_core::IUnknown);
impl WiFiDirectConnectionListener {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectConnectionListener, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WiFiDirectConnectionListener, WiFiDirectConnectionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionRequested)(::windows_core::Interface::as_raw(this), token).ok() }
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
impl ::windows_core::RuntimeType for WiFiDirectConnectionListener {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionListener;{699c1b0d-8d13-4ee9-b9ec-9c72f8251f7d})");
}
impl ::core::clone::Clone for WiFiDirectConnectionListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionListener {
    type Vtable = IWiFiDirectConnectionListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectConnectionListener {
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionListener as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionListener {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionListener";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectConnectionListener, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectConnectionListener {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionListener {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionParameters(::windows_core::IUnknown);
impl WiFiDirectConnectionParameters {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectConnectionParameters, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn GroupOwnerIntent(&self) -> ::windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GroupOwnerIntent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGroupOwnerIntent(&self, value: i16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroupOwnerIntent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PreferenceOrderedConfigurationMethods(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectConfigurationMethod>> {
        let this = &::windows_core::ComInterface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferenceOrderedConfigurationMethods)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreferredPairingProcedure(&self) -> ::windows_core::Result<WiFiDirectPairingProcedure> {
        let this = &::windows_core::ComInterface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredPairingProcedure)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreferredPairingProcedure(&self, value: WiFiDirectPairingProcedure) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWiFiDirectConnectionParameters2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredPairingProcedure)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn GetDevicePairingKinds(configurationmethod: WiFiDirectConfigurationMethod) -> ::windows_core::Result<super::Enumeration::DevicePairingKinds> {
        Self::IWiFiDirectConnectionParametersStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDevicePairingKinds)(::windows_core::Interface::as_raw(this), configurationmethod, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectConnectionParametersStatics<R, F: FnOnce(&IWiFiDirectConnectionParametersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectConnectionParameters, IWiFiDirectConnectionParametersStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WiFiDirectConnectionParameters {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters;{b2e55405-5702-4b16-a02c-bbcd21ef6098})");
}
impl ::core::clone::Clone for WiFiDirectConnectionParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionParameters {
    type Vtable = IWiFiDirectConnectionParameters_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectConnectionParameters {
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionParameters as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionParameters {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionParameters";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectConnectionParameters, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Devices_Enumeration")]
impl ::windows_core::CanTryInto<super::Enumeration::IDevicePairingSettings> for WiFiDirectConnectionParameters {}
unsafe impl ::core::marker::Send for WiFiDirectConnectionParameters {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionParameters {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionRequest(::windows_core::IUnknown);
impl WiFiDirectConnectionRequest {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for WiFiDirectConnectionRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest;{8eb99605-914f-49c3-a614-d18dc5b19b43})");
}
impl ::core::clone::Clone for WiFiDirectConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionRequest {
    type Vtable = IWiFiDirectConnectionRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectConnectionRequest {
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequest";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectConnectionRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for WiFiDirectConnectionRequest {}
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequest {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequest {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectConnectionRequestedEventArgs(::windows_core::IUnknown);
impl WiFiDirectConnectionRequestedEventArgs {
    pub fn GetConnectionRequest(&self) -> ::windows_core::Result<WiFiDirectConnectionRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for WiFiDirectConnectionRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs;{f99d20be-d38d-484f-8215-e7b65abf244c})");
}
impl ::core::clone::Clone for WiFiDirectConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectConnectionRequestedEventArgs {
    type Vtable = IWiFiDirectConnectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectConnectionRequestedEventArgs {
    const IID: ::windows_core::GUID = <IWiFiDirectConnectionRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectConnectionRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectConnectionRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for WiFiDirectConnectionRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectDevice(::windows_core::IUnknown);
impl WiFiDirectDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows_core::Result<WiFiDirectConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<WiFiDirectDevice, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveConnectionStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetConnectionEndpointPairs(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::EndpointPair>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetConnectionEndpointPairs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>> {
        Self::IWiFiDirectDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector2(r#type: WiFiDirectDeviceSelectorType) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync2<P0>(deviceid: &::windows_core::HSTRING, connectionparameters: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<WiFiDirectDevice>>
    where
        P0: ::windows_core::IntoParam<WiFiDirectConnectionParameters>,
    {
        Self::IWiFiDirectDeviceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), connectionparameters.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics<R, F: FnOnce(&IWiFiDirectDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWiFiDirectDeviceStatics2<R, F: FnOnce(&IWiFiDirectDeviceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectDevice, IWiFiDirectDeviceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WiFiDirectDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectDevice;{72deaaa8-72eb-4dae-8a28-8513355d2777})");
}
impl ::core::clone::Clone for WiFiDirectDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectDevice {
    type Vtable = IWiFiDirectDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectDevice {
    const IID: ::windows_core::GUID = <IWiFiDirectDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectDevice {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectDevice";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for WiFiDirectDevice {}
unsafe impl ::core::marker::Send for WiFiDirectDevice {}
unsafe impl ::core::marker::Sync for WiFiDirectDevice {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectInformationElement(::windows_core::IUnknown);
impl WiFiDirectInformationElement {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectInformationElement, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Oui(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Oui)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetOui<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOui)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn OuiType(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OuiType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOuiType(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOuiType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetValue<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<P0>(buffer: P0) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections"))]
    pub fn CreateFromDeviceInformation<P0>(deviceinformation: P0) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<WiFiDirectInformationElement>>
    where
        P0: ::windows_core::IntoParam<super::Enumeration::DeviceInformation>,
    {
        Self::IWiFiDirectInformationElementStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceInformation)(::windows_core::Interface::as_raw(this), deviceinformation.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiDirectInformationElementStatics<R, F: FnOnce(&IWiFiDirectInformationElementStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiDirectInformationElement, IWiFiDirectInformationElementStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WiFiDirectInformationElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectInformationElement;{affb72d6-76bb-497e-ac8b-dc72838bc309})");
}
impl ::core::clone::Clone for WiFiDirectInformationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectInformationElement {
    type Vtable = IWiFiDirectInformationElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectInformationElement {
    const IID: ::windows_core::GUID = <IWiFiDirectInformationElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectInformationElement {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectInformationElement";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectInformationElement, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectInformationElement {}
unsafe impl ::core::marker::Sync for WiFiDirectInformationElement {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
pub struct WiFiDirectLegacySettings(::windows_core::IUnknown);
impl WiFiDirectLegacySettings {
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ssid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ssid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSsid(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSsid)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Passphrase(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Passphrase)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPassphrase<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassphrase)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
impl ::windows_core::RuntimeType for WiFiDirectLegacySettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFiDirect.WiFiDirectLegacySettings;{a64fdbba-f2fd-4567-a91b-f5c2f5321057})");
}
impl ::core::clone::Clone for WiFiDirectLegacySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WiFiDirectLegacySettings {
    type Vtable = IWiFiDirectLegacySettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiDirectLegacySettings {
    const IID: ::windows_core::GUID = <IWiFiDirectLegacySettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiDirectLegacySettings {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.WiFiDirectLegacySettings";
}
::windows_core::imp::interface_hierarchy!(WiFiDirectLegacySettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WiFiDirectLegacySettings {}
unsafe impl ::core::marker::Sync for WiFiDirectLegacySettings {}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectAdvertisementListenStateDiscoverability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectAdvertisementListenStateDiscoverability {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectAdvertisementListenStateDiscoverability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementListenStateDiscoverability").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectAdvertisementListenStateDiscoverability {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementListenStateDiscoverability;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectAdvertisementPublisherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectAdvertisementPublisherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectAdvertisementPublisherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectAdvertisementPublisherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectAdvertisementPublisherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectAdvertisementPublisherStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectConfigurationMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectConfigurationMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectConfigurationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConfigurationMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectConfigurationMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConfigurationMethod;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectConnectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectConnectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectConnectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectConnectionStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectDeviceSelectorType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectDeviceSelectorType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectDeviceSelectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectDeviceSelectorType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectDeviceSelectorType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectDeviceSelectorType;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectError {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectError {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectError").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectError;i4)");
}
#[doc = "*Required features: `\"Devices_WiFiDirect\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WiFiDirectPairingProcedure {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WiFiDirectPairingProcedure {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WiFiDirectPairingProcedure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiDirectPairingProcedure").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WiFiDirectPairingProcedure {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFiDirect.WiFiDirectPairingProcedure;i4)");
}
