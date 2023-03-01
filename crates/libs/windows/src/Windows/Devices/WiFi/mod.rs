#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapter {
    type Vtable = IWiFiAdapter_Vtbl;
}
impl ::core::clone::Clone for IWiFiAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiAdapter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    NetworkAdapter: usize,
    #[cfg(feature = "Foundation")]
    pub ScanAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScanAsync: usize,
    pub NetworkReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AvailableNetworksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailableNetworksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailableNetworksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailableNetworksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAndSsidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, ssid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAndSsidAsync: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapter2 {
    type Vtable = IWiFiAdapter2_Vtbl;
}
impl ::core::clone::Clone for IWiFiAdapter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiAdapter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc4501d_81e4_453d_9430_1fcafbadd6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetWpsConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetWpsConfigurationAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: *mut ::core::ffi::c_void, reconnectionkind: WiFiReconnectionKind, passwordcredential: *mut ::core::ffi::c_void, ssid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAdapterStatics {
    type Vtable = IWiFiAdapterStatics_Vtbl;
}
impl ::core::clone::Clone for IWiFiAdapterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiAdapterStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda25fddd_d24c_43e3_aabd_c4659f730f99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAdaptersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAdaptersAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAvailableNetwork(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_Vtbl;
}
impl ::core::clone::Clone for IWiFiAvailableNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiAvailableNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uptime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uptime: usize,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Bssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ChannelCenterFrequencyInKilohertz: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub NetworkRssiInDecibelMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SignalBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub NetworkKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiNetworkKind) -> ::windows::core::HRESULT,
    pub PhyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiPhyKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub SecuritySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    SecuritySettings: usize,
    #[cfg(feature = "Foundation")]
    pub BeaconInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeaconInterval: usize,
    pub IsWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiConnectionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_Vtbl;
}
impl ::core::clone::Clone for IWiFiConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiConnectionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiConnectionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiConnectionStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiNetworkReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_Vtbl;
}
impl ::core::clone::Clone for IWiFiNetworkReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiNetworkReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiNetworkReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableNetworks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiOnDemandHotspotConnectTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiOnDemandHotspotConnectTriggerDetails {
    type Vtable = IWiFiOnDemandHotspotConnectTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IWiFiOnDemandHotspotConnectTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiOnDemandHotspotConnectTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa268eb58_68f5_59cf_8d38_35bf44b097ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiOnDemandHotspotConnectTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestedNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WiFiOnDemandHotspotConnectStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectAsync: usize,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiOnDemandHotspotConnectionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiOnDemandHotspotConnectionResult {
    type Vtable = IWiFiOnDemandHotspotConnectionResult_Vtbl;
}
impl ::core::clone::Clone for IWiFiOnDemandHotspotConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiOnDemandHotspotConnectionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x911794a1_6c82_5de3_8a4a_f9ff22a4957a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiOnDemandHotspotConnectionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiOnDemandHotspotConnectStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiOnDemandHotspotNetwork(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiOnDemandHotspotNetwork {
    type Vtable = IWiFiOnDemandHotspotNetwork_Vtbl;
}
impl ::core::clone::Clone for IWiFiOnDemandHotspotNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiOnDemandHotspotNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18dc7115_a04e_507c_bbaf_b78369d29fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiOnDemandHotspotNetwork_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newproperties: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiOnDemandHotspotNetworkProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiOnDemandHotspotNetworkProperties {
    type Vtable = IWiFiOnDemandHotspotNetworkProperties_Vtbl;
}
impl ::core::clone::Clone for IWiFiOnDemandHotspotNetworkProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiOnDemandHotspotNetworkProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc810a1f2_c81d_5852_be50_e4bd4d81e98d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiOnDemandHotspotNetworkProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Availability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiOnDemandHotspotAvailability) -> ::windows::core::HRESULT,
    pub SetAvailability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WiFiOnDemandHotspotAvailability) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemainingBatteryPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingBatteryPercent: usize,
    #[cfg(feature = "Foundation")]
    pub SetRemainingBatteryPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRemainingBatteryPercent: usize,
    #[cfg(feature = "Foundation")]
    pub CellularBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CellularBars: usize,
    #[cfg(feature = "Foundation")]
    pub SetCellularBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCellularBars: usize,
    pub IsMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMetered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Password: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetPassword: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiOnDemandHotspotNetworkStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiOnDemandHotspotNetworkStatics {
    type Vtable = IWiFiOnDemandHotspotNetworkStatics_Vtbl;
}
impl ::core::clone::Clone for IWiFiOnDemandHotspotNetworkStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiOnDemandHotspotNetworkStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f5b8ac_80e7_5054_871c_8739f374e3c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiOnDemandHotspotNetworkStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetOrCreateById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiWpsConfigurationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_Vtbl;
}
impl ::core::clone::Clone for IWiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWiFiWpsConfigurationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedWpsKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedWpsKinds: usize,
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiAdapter(::windows::core::IUnknown);
impl WiFiAdapter {
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn NetworkAdapter(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Networking::Connectivity::NetworkAdapter>();
            (::windows::core::Interface::vtable(this).NetworkAdapter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScanAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ScanAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkReport(&self) -> ::windows::core::Result<WiFiNetworkReport> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiNetworkReport>();
            (::windows::core::Interface::vtable(this).NetworkReport)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailableNetworksChanged(&self, args: &super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).AvailableNetworksChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(args), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailableNetworksChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailableNetworksChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self, availablenetwork: &WiFiAvailableNetwork, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(availablenetwork), reconnectionkind, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAsync(&self, availablenetwork: &WiFiAvailableNetwork, reconnectionkind: WiFiReconnectionKind, passwordcredential: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>();
            (::windows::core::Interface::vtable(this).ConnectWithPasswordCredentialAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(availablenetwork), reconnectionkind, ::core::mem::transmute_copy(passwordcredential), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAndSsidAsync(&self, availablenetwork: &WiFiAvailableNetwork, reconnectionkind: WiFiReconnectionKind, passwordcredential: &super::super::Security::Credentials::PasswordCredential, ssid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>();
            (::windows::core::Interface::vtable(this).ConnectWithPasswordCredentialAndSsidAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(availablenetwork), reconnectionkind, ::core::mem::transmute_copy(passwordcredential), ::core::mem::transmute_copy(ssid), &mut result__).from_abi(result__)
        }
    }
    pub fn Disconnect(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Disconnect)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetWpsConfigurationAsync(&self, availablenetwork: &WiFiAvailableNetwork) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>> {
        let this = &::windows::core::ComInterface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>();
            (::windows::core::Interface::vtable(this).GetWpsConfigurationAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(availablenetwork), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(&self, availablenetwork: &WiFiAvailableNetwork, reconnectionkind: WiFiReconnectionKind, passwordcredential: &super::super::Security::Credentials::PasswordCredential, ssid: &::windows::core::HSTRING, connectionmethod: WiFiConnectionMethod) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = &::windows::core::ComInterface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>();
            (::windows::core::Interface::vtable(this).ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(availablenetwork), reconnectionkind, ::core::mem::transmute_copy(passwordcredential), ::core::mem::transmute_copy(ssid), connectionmethod, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAdaptersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>();
            (::windows::core::Interface::vtable(this).FindAllAdaptersAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiAdapter>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiAdapterStatics<R, F: FnOnce(&IWiFiAdapterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WiFiAdapter, IWiFiAdapterStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WiFiAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAdapter {}
impl ::core::fmt::Debug for WiFiAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAdapter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiAdapter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAdapter;{a6c4e423-3d75-43a4-b9de-11e26b72d9b0})");
}
impl ::core::clone::Clone for WiFiAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiAdapter {
    type Vtable = IWiFiAdapter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiAdapter {
    const IID: ::windows::core::GUID = <IWiFiAdapter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAdapter";
}
::windows::imp::interface_hierarchy!(WiFiAdapter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiAdapter {}
unsafe impl ::core::marker::Sync for WiFiAdapter {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiAvailableNetwork(::windows::core::IUnknown);
impl WiFiAvailableNetwork {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uptime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Uptime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Ssid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Bssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Bssid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ChannelCenterFrequencyInKilohertz)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).NetworkRssiInDecibelMilliwatts)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SignalBars(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u8>();
            (::windows::core::Interface::vtable(this).SignalBars)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkKind(&self) -> ::windows::core::Result<WiFiNetworkKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiNetworkKind>();
            (::windows::core::Interface::vtable(this).NetworkKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PhyKind(&self) -> ::windows::core::Result<WiFiPhyKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiPhyKind>();
            (::windows::core::Interface::vtable(this).PhyKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn SecuritySettings(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Networking::Connectivity::NetworkSecuritySettings>();
            (::windows::core::Interface::vtable(this).SecuritySettings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeaconInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).BeaconInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsWiFiDirect(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWiFiDirect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiAvailableNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAvailableNetwork {}
impl ::core::fmt::Debug for WiFiAvailableNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAvailableNetwork").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiAvailableNetwork {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAvailableNetwork;{26e96246-183e-4704-9826-71b4a2f0f668})");
}
impl ::core::clone::Clone for WiFiAvailableNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiAvailableNetwork {
    const IID: ::windows::core::GUID = <IWiFiAvailableNetwork as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAvailableNetwork";
}
::windows::imp::interface_hierarchy!(WiFiAvailableNetwork, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiAvailableNetwork {}
unsafe impl ::core::marker::Sync for WiFiAvailableNetwork {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiConnectionResult(::windows::core::IUnknown);
impl WiFiConnectionResult {
    pub fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiConnectionStatus>();
            (::windows::core::Interface::vtable(this).ConnectionStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiConnectionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiConnectionResult {}
impl ::core::fmt::Debug for WiFiConnectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiConnectionResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiConnectionResult;{143bdfd9-c37d-40be-a5c8-857bce85a931})");
}
impl ::core::clone::Clone for WiFiConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiConnectionResult {
    const IID: ::windows::core::GUID = <IWiFiConnectionResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiConnectionResult";
}
::windows::imp::interface_hierarchy!(WiFiConnectionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiConnectionResult {}
unsafe impl ::core::marker::Sync for WiFiConnectionResult {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiNetworkReport(::windows::core::IUnknown);
impl WiFiNetworkReport {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableNetworks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>();
            (::windows::core::Interface::vtable(this).AvailableNetworks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiNetworkReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiNetworkReport {}
impl ::core::fmt::Debug for WiFiNetworkReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiNetworkReport").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiNetworkReport {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiNetworkReport;{9524ded2-5911-445e-8194-be4f1a704895})");
}
impl ::core::clone::Clone for WiFiNetworkReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiNetworkReport {
    const IID: ::windows::core::GUID = <IWiFiNetworkReport as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiNetworkReport";
}
::windows::imp::interface_hierarchy!(WiFiNetworkReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiNetworkReport {}
unsafe impl ::core::marker::Sync for WiFiNetworkReport {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotConnectTriggerDetails(::windows::core::IUnknown);
impl WiFiOnDemandHotspotConnectTriggerDetails {
    pub fn RequestedNetwork(&self) -> ::windows::core::Result<WiFiOnDemandHotspotNetwork> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotNetwork>();
            (::windows::core::Interface::vtable(this).RequestedNetwork)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportError(&self, status: WiFiOnDemandHotspotConnectStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::windows::core::Interface::as_raw(this), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiOnDemandHotspotConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<WiFiOnDemandHotspotConnectionResult>>();
            (::windows::core::Interface::vtable(this).ConnectAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Connect(&self) -> ::windows::core::Result<WiFiOnDemandHotspotConnectionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotConnectionResult>();
            (::windows::core::Interface::vtable(this).Connect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotConnectTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotConnectTriggerDetails {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotConnectTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotConnectTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotConnectTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails;{a268eb58-68f5-59cf-8d38-35bf44b097ef})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotConnectTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotConnectTriggerDetails {
    type Vtable = IWiFiOnDemandHotspotConnectTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotConnectTriggerDetails {
    const IID: ::windows::core::GUID = <IWiFiOnDemandHotspotConnectTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotConnectTriggerDetails {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiOnDemandHotspotConnectTriggerDetails";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotConnectTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotConnectTriggerDetails {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotConnectTriggerDetails {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotConnectionResult(::windows::core::IUnknown);
impl WiFiOnDemandHotspotConnectionResult {
    pub fn Status(&self) -> ::windows::core::Result<WiFiOnDemandHotspotConnectStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotConnectStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotConnectionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotConnectionResult {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotConnectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotConnectionResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotConnectionResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult;{911794a1-6c82-5de3-8a4a-f9ff22a4957a})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotConnectionResult {
    type Vtable = IWiFiOnDemandHotspotConnectionResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotConnectionResult {
    const IID: ::windows::core::GUID = <IWiFiOnDemandHotspotConnectionResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiOnDemandHotspotConnectionResult";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotConnectionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotConnectionResult {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotConnectionResult {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotNetwork(::windows::core::IUnknown);
impl WiFiOnDemandHotspotNetwork {
    pub fn GetProperties(&self) -> ::windows::core::Result<WiFiOnDemandHotspotNetworkProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotNetworkProperties>();
            (::windows::core::Interface::vtable(this).GetProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UpdateProperties(&self, newproperties: &WiFiOnDemandHotspotNetworkProperties) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateProperties)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(newproperties)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetOrCreateById(networkid: ::windows::core::GUID) -> ::windows::core::Result<WiFiOnDemandHotspotNetwork> {
        Self::IWiFiOnDemandHotspotNetworkStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotNetwork>();
            (::windows::core::Interface::vtable(this).GetOrCreateById)(::windows::core::Interface::as_raw(this), networkid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWiFiOnDemandHotspotNetworkStatics<R, F: FnOnce(&IWiFiOnDemandHotspotNetworkStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WiFiOnDemandHotspotNetwork, IWiFiOnDemandHotspotNetworkStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotNetwork {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotNetwork").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotNetwork {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork;{18dc7115-a04e-507c-bbaf-b78369d29fa7})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotNetwork {
    type Vtable = IWiFiOnDemandHotspotNetwork_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotNetwork {
    const IID: ::windows::core::GUID = <IWiFiOnDemandHotspotNetwork as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiOnDemandHotspotNetwork";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotNetwork, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotNetwork {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotNetwork {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotNetworkProperties(::windows::core::IUnknown);
impl WiFiOnDemandHotspotNetworkProperties {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Availability(&self) -> ::windows::core::Result<WiFiOnDemandHotspotAvailability> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiOnDemandHotspotAvailability>();
            (::windows::core::Interface::vtable(this).Availability)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAvailability(&self, value: WiFiOnDemandHotspotAvailability) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAvailability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingBatteryPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<u32>>();
            (::windows::core::Interface::vtable(this).RemainingBatteryPercent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRemainingBatteryPercent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemainingBatteryPercent)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CellularBars(&self) -> ::windows::core::Result<super::super::Foundation::IReference<WiFiOnDemandHotspotCellularBars>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<WiFiOnDemandHotspotCellularBars>>();
            (::windows::core::Interface::vtable(this).CellularBars)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCellularBars<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<WiFiOnDemandHotspotCellularBars>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCellularBars)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsMetered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsMetered)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsMetered(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMetered)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Ssid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSsid(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSsid)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Password(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Credentials::PasswordCredential>();
            (::windows::core::Interface::vtable(this).Password)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetPassword(&self, value: &super::super::Security::Credentials::PasswordCredential) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPassword)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotNetworkProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotNetworkProperties {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotNetworkProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotNetworkProperties").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotNetworkProperties {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties;{c810a1f2-c81d-5852-be50-e4bd4d81e98d})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotNetworkProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotNetworkProperties {
    type Vtable = IWiFiOnDemandHotspotNetworkProperties_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotNetworkProperties {
    const IID: ::windows::core::GUID = <IWiFiOnDemandHotspotNetworkProperties as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotNetworkProperties {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiOnDemandHotspotNetworkProperties";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotNetworkProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotNetworkProperties {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotNetworkProperties {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(::windows::core::IUnknown);
impl WiFiWpsConfigurationResult {
    pub fn Status(&self) -> ::windows::core::Result<WiFiWpsConfigurationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WiFiWpsConfigurationStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedWpsKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>();
            (::windows::core::Interface::vtable(this).SupportedWpsKinds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WiFiWpsConfigurationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiWpsConfigurationResult {}
impl ::core::fmt::Debug for WiFiWpsConfigurationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsConfigurationResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiWpsConfigurationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiWpsConfigurationResult;{67b49871-17ee-42d1-b14f-5a11f1226fb5})");
}
impl ::core::clone::Clone for WiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiWpsConfigurationResult {
    const IID: ::windows::core::GUID = <IWiFiWpsConfigurationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiWpsConfigurationResult";
}
::windows::imp::interface_hierarchy!(WiFiWpsConfigurationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WiFiWpsConfigurationResult {}
unsafe impl ::core::marker::Sync for WiFiWpsConfigurationResult {}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiAccessStatus {}
impl ::core::clone::Clone for WiFiAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiAccessStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAccessStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiAccessStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiAccessStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: Self = Self(0i32);
    pub const WpsPin: Self = Self(1i32);
    pub const WpsPushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiConnectionMethod {}
impl ::core::clone::Clone for WiFiConnectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiConnectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiConnectionMethod {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiConnectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionMethod").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiConnectionMethod {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionMethod;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AccessRevoked: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(6i32);
}
impl ::core::marker::Copy for WiFiConnectionStatus {}
impl ::core::clone::Clone for WiFiConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiConnectionStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiConnectionStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: Self = Self(0i32);
    pub const Infrastructure: Self = Self(1i32);
    pub const Adhoc: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiNetworkKind {}
impl ::core::clone::Clone for WiFiNetworkKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiNetworkKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiNetworkKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiNetworkKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiNetworkKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiNetworkKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiNetworkKind;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiOnDemandHotspotAvailability(pub i32);
impl WiFiOnDemandHotspotAvailability {
    pub const Available: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiOnDemandHotspotAvailability {}
impl ::core::clone::Clone for WiFiOnDemandHotspotAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiOnDemandHotspotAvailability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiOnDemandHotspotAvailability {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiOnDemandHotspotAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotAvailability").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotAvailability {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiOnDemandHotspotAvailability;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiOnDemandHotspotCellularBars(pub i32);
impl WiFiOnDemandHotspotCellularBars {
    pub const ZeroBars: Self = Self(0i32);
    pub const OneBar: Self = Self(1i32);
    pub const TwoBars: Self = Self(2i32);
    pub const ThreeBars: Self = Self(3i32);
    pub const FourBars: Self = Self(4i32);
    pub const FiveBars: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiOnDemandHotspotCellularBars {}
impl ::core::clone::Clone for WiFiOnDemandHotspotCellularBars {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiOnDemandHotspotCellularBars {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiOnDemandHotspotCellularBars {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiOnDemandHotspotCellularBars {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotCellularBars").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotCellularBars {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiOnDemandHotspotCellularBars;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiOnDemandHotspotConnectStatus(pub i32);
impl WiFiOnDemandHotspotConnectStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AppTimedOut: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(5i32);
    pub const BluetoothConnectFailed: Self = Self(6i32);
    pub const BluetoothTransmissionError: Self = Self(7i32);
    pub const OperationCanceledByUser: Self = Self(8i32);
    pub const EntitlementCheckFailed: Self = Self(9i32);
    pub const NoCellularSignal: Self = Self(10i32);
    pub const CellularDataTurnedOff: Self = Self(11i32);
    pub const WlanConnectFailed: Self = Self(12i32);
    pub const WlanNotVisible: Self = Self(13i32);
    pub const AccessPointCannotConnect: Self = Self(14i32);
    pub const CellularConnectTimedOut: Self = Self(15i32);
    pub const RoamingNotAllowed: Self = Self(16i32);
    pub const PairingRequired: Self = Self(17i32);
    pub const DataLimitReached: Self = Self(18i32);
}
impl ::core::marker::Copy for WiFiOnDemandHotspotConnectStatus {}
impl ::core::clone::Clone for WiFiOnDemandHotspotConnectStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiOnDemandHotspotConnectStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiOnDemandHotspotConnectStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiOnDemandHotspotConnectStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotConnectStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotConnectStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiOnDemandHotspotConnectStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: Self = Self(0i32);
    pub const Fhss: Self = Self(1i32);
    pub const Dsss: Self = Self(2i32);
    pub const IRBaseband: Self = Self(3i32);
    pub const Ofdm: Self = Self(4i32);
    pub const Hrdsss: Self = Self(5i32);
    pub const Erp: Self = Self(6i32);
    pub const HT: Self = Self(7i32);
    pub const Vht: Self = Self(8i32);
    pub const Dmg: Self = Self(9i32);
    pub const HE: Self = Self(10i32);
    pub const Eht: Self = Self(11i32);
}
impl ::core::marker::Copy for WiFiPhyKind {}
impl ::core::clone::Clone for WiFiPhyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiPhyKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiPhyKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiPhyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiPhyKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiPhyKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiPhyKind;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiReconnectionKind {}
impl ::core::clone::Clone for WiFiReconnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiReconnectionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiReconnectionKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiReconnectionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiReconnectionKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiReconnectionKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiReconnectionKind;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiWpsConfigurationStatus {}
impl ::core::clone::Clone for WiFiWpsConfigurationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiWpsConfigurationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiWpsConfigurationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiWpsConfigurationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsConfigurationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiWpsConfigurationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsConfigurationStatus;i4)");
}
#[doc = "*Required features: `\"Devices_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Ethernet: Self = Self(4i32);
    pub const Usb: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiWpsKind {}
impl ::core::clone::Clone for WiFiWpsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiWpsKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WiFiWpsKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WiFiWpsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiWpsKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
