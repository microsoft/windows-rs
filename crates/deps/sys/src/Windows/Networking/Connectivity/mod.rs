#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AttributedNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Pap: Self = Self(1i32);
    pub const Chap: Self = Self(2i32);
    pub const Mschapv2: Self = Self(3i32);
}
#[repr(transparent)]
pub struct CellularApnContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionCost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ConnectionProfileFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectivityInterval(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPlanStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPlanUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: Self = Self(0i32);
    pub const PerHour: Self = Self(1i32);
    pub const PerDay: Self = Self(2i32);
    pub const Total: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
    pub const Authenticated: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IAttributedNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICellularApnContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICellularApnContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionCost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionCost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfile3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfile4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfile5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfileFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfileFilter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionProfileFilter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectivityInterval(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectivityManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPlanStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPlanUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIPInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanIdentifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILanIdentifierData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkInformationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkSecuritySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkStateChangeEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkStateChangeEventDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProxyConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoutePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRoutePolicyFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWlanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWwanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWwanConnectionProfileDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LanIdentifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LanIdentifierData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkAdapter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Open80211: Self = Self(2i32);
    pub const SharedKey80211: Self = Self(3i32);
    pub const Wpa: Self = Self(4i32);
    pub const WpaPsk: Self = Self(5i32);
    pub const WpaNone: Self = Self(6i32);
    pub const Rsna: Self = Self(7i32);
    pub const RsnaPsk: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Wpa3: Self = Self(10i32);
    pub const Wpa3Enterprise192Bits: Self = Self(10i32);
    pub const Wpa3Sae: Self = Self(11i32);
    pub const Owe: Self = Self(12i32);
    pub const Wpa3Enterprise: Self = Self(13i32);
}
#[repr(transparent)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const LocalAccess: Self = Self(1i32);
    pub const ConstrainedInternetAccess: Self = Self(2i32);
    pub const InternetAccess: Self = Self(3i32);
}
#[repr(transparent)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: Self = Self(0i32);
    pub const Unrestricted: Self = Self(1i32);
    pub const Fixed: Self = Self(2i32);
    pub const Variable: Self = Self(3i32);
}
#[repr(transparent)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Wep: Self = Self(2i32);
    pub const Wep40: Self = Self(3i32);
    pub const Wep104: Self = Self(4i32);
    pub const Tkip: Self = Self(5i32);
    pub const Ccmp: Self = Self(6i32);
    pub const WpaUseGroup: Self = Self(7i32);
    pub const RsnUseGroup: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Gcmp: Self = Self(10i32);
    pub const Gcmp256: Self = Self(11i32);
}
#[repr(transparent)]
pub struct NetworkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkSecuritySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkStateChangeEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkStatusChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: Self = Self(0u32);
    pub const Internet: Self = Self(1u32);
    pub const PrivateNetwork: Self = Self(2u32);
}
#[repr(transparent)]
pub struct NetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NetworkUsageStates(i32);
#[repr(transparent)]
pub struct ProviderNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProxyConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: Self = Self(0u32);
    pub const NotRoaming: Self = Self(1u32);
    pub const Roaming: Self = Self(2u32);
}
#[repr(transparent)]
pub struct RoutePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: Self = Self(0i32);
    pub const No: Self = Self(1i32);
    pub const Yes: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WlanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WwanContract(i32);
#[repr(transparent)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
#[repr(transparent)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Ipv4v6: Self = Self(3i32);
    pub const Ipv4v6v4Xlat: Self = Self(4i32);
}
#[repr(transparent)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
