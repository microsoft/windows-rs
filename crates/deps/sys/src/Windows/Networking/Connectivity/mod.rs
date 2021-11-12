#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AttributedNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: CellularApnAuthenticationType = CellularApnAuthenticationType(0i32);
    pub const Pap: CellularApnAuthenticationType = CellularApnAuthenticationType(1i32);
    pub const Chap: CellularApnAuthenticationType = CellularApnAuthenticationType(2i32);
    pub const Mschapv2: CellularApnAuthenticationType = CellularApnAuthenticationType(3i32);
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
    pub const Success: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(0i32);
    pub const DeniedByUser: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(1i32);
    pub const DeniedBySystem: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(2i32);
    pub const UnknownError: ConnectionProfileDeleteStatus = ConnectionProfileDeleteStatus(3i32);
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
    pub const PerMinute: DataUsageGranularity = DataUsageGranularity(0i32);
    pub const PerHour: DataUsageGranularity = DataUsageGranularity(1i32);
    pub const PerDay: DataUsageGranularity = DataUsageGranularity(2i32);
    pub const Total: DataUsageGranularity = DataUsageGranularity(3i32);
}
#[repr(transparent)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: DomainConnectivityLevel = DomainConnectivityLevel(0i32);
    pub const Unauthenticated: DomainConnectivityLevel = DomainConnectivityLevel(1i32);
    pub const Authenticated: DomainConnectivityLevel = DomainConnectivityLevel(2i32);
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
    pub const None: NetworkAuthenticationType = NetworkAuthenticationType(0i32);
    pub const Unknown: NetworkAuthenticationType = NetworkAuthenticationType(1i32);
    pub const Open80211: NetworkAuthenticationType = NetworkAuthenticationType(2i32);
    pub const SharedKey80211: NetworkAuthenticationType = NetworkAuthenticationType(3i32);
    pub const Wpa: NetworkAuthenticationType = NetworkAuthenticationType(4i32);
    pub const WpaPsk: NetworkAuthenticationType = NetworkAuthenticationType(5i32);
    pub const WpaNone: NetworkAuthenticationType = NetworkAuthenticationType(6i32);
    pub const Rsna: NetworkAuthenticationType = NetworkAuthenticationType(7i32);
    pub const RsnaPsk: NetworkAuthenticationType = NetworkAuthenticationType(8i32);
    pub const Ihv: NetworkAuthenticationType = NetworkAuthenticationType(9i32);
    pub const Wpa3: NetworkAuthenticationType = NetworkAuthenticationType(10i32);
    pub const Wpa3Enterprise192Bits: NetworkAuthenticationType = NetworkAuthenticationType(10i32);
    pub const Wpa3Sae: NetworkAuthenticationType = NetworkAuthenticationType(11i32);
    pub const Owe: NetworkAuthenticationType = NetworkAuthenticationType(12i32);
    pub const Wpa3Enterprise: NetworkAuthenticationType = NetworkAuthenticationType(13i32);
}
#[repr(transparent)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: NetworkConnectivityLevel = NetworkConnectivityLevel(0i32);
    pub const LocalAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(1i32);
    pub const ConstrainedInternetAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(2i32);
    pub const InternetAccess: NetworkConnectivityLevel = NetworkConnectivityLevel(3i32);
}
#[repr(transparent)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: NetworkCostType = NetworkCostType(0i32);
    pub const Unrestricted: NetworkCostType = NetworkCostType(1i32);
    pub const Fixed: NetworkCostType = NetworkCostType(2i32);
    pub const Variable: NetworkCostType = NetworkCostType(3i32);
}
#[repr(transparent)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: NetworkEncryptionType = NetworkEncryptionType(0i32);
    pub const Unknown: NetworkEncryptionType = NetworkEncryptionType(1i32);
    pub const Wep: NetworkEncryptionType = NetworkEncryptionType(2i32);
    pub const Wep40: NetworkEncryptionType = NetworkEncryptionType(3i32);
    pub const Wep104: NetworkEncryptionType = NetworkEncryptionType(4i32);
    pub const Tkip: NetworkEncryptionType = NetworkEncryptionType(5i32);
    pub const Ccmp: NetworkEncryptionType = NetworkEncryptionType(6i32);
    pub const WpaUseGroup: NetworkEncryptionType = NetworkEncryptionType(7i32);
    pub const RsnUseGroup: NetworkEncryptionType = NetworkEncryptionType(8i32);
    pub const Ihv: NetworkEncryptionType = NetworkEncryptionType(9i32);
    pub const Gcmp: NetworkEncryptionType = NetworkEncryptionType(10i32);
    pub const Gcmp256: NetworkEncryptionType = NetworkEncryptionType(11i32);
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
    pub const None: NetworkTypes = NetworkTypes(0u32);
    pub const Internet: NetworkTypes = NetworkTypes(1u32);
    pub const PrivateNetwork: NetworkTypes = NetworkTypes(2u32);
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
    pub const None: RoamingStates = RoamingStates(0u32);
    pub const NotRoaming: RoamingStates = RoamingStates(1u32);
    pub const Roaming: RoamingStates = RoamingStates(2u32);
}
#[repr(transparent)]
pub struct RoutePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: TriStates = TriStates(0i32);
    pub const No: TriStates = TriStates(1i32);
    pub const Yes: TriStates = TriStates(2i32);
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
    pub const None: WwanDataClass = WwanDataClass(0u32);
    pub const Gprs: WwanDataClass = WwanDataClass(1u32);
    pub const Edge: WwanDataClass = WwanDataClass(2u32);
    pub const Umts: WwanDataClass = WwanDataClass(4u32);
    pub const Hsdpa: WwanDataClass = WwanDataClass(8u32);
    pub const Hsupa: WwanDataClass = WwanDataClass(16u32);
    pub const LteAdvanced: WwanDataClass = WwanDataClass(32u32);
    pub const Cdma1xRtt: WwanDataClass = WwanDataClass(65536u32);
    pub const Cdma1xEvdo: WwanDataClass = WwanDataClass(131072u32);
    pub const Cdma1xEvdoRevA: WwanDataClass = WwanDataClass(262144u32);
    pub const Cdma1xEvdv: WwanDataClass = WwanDataClass(524288u32);
    pub const Cdma3xRtt: WwanDataClass = WwanDataClass(1048576u32);
    pub const Cdma1xEvdoRevB: WwanDataClass = WwanDataClass(2097152u32);
    pub const CdmaUmb: WwanDataClass = WwanDataClass(4194304u32);
    pub const Custom: WwanDataClass = WwanDataClass(2147483648u32);
}
#[repr(transparent)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: WwanNetworkIPKind = WwanNetworkIPKind(0i32);
    pub const Ipv4: WwanNetworkIPKind = WwanNetworkIPKind(1i32);
    pub const Ipv6: WwanNetworkIPKind = WwanNetworkIPKind(2i32);
    pub const Ipv4v6: WwanNetworkIPKind = WwanNetworkIPKind(3i32);
    pub const Ipv4v6v4Xlat: WwanNetworkIPKind = WwanNetworkIPKind(4i32);
}
#[repr(transparent)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: WwanNetworkRegistrationState = WwanNetworkRegistrationState(0i32);
    pub const Deregistered: WwanNetworkRegistrationState = WwanNetworkRegistrationState(1i32);
    pub const Searching: WwanNetworkRegistrationState = WwanNetworkRegistrationState(2i32);
    pub const Home: WwanNetworkRegistrationState = WwanNetworkRegistrationState(3i32);
    pub const Roaming: WwanNetworkRegistrationState = WwanNetworkRegistrationState(4i32);
    pub const Partner: WwanNetworkRegistrationState = WwanNetworkRegistrationState(5i32);
    pub const Denied: WwanNetworkRegistrationState = WwanNetworkRegistrationState(6i32);
}
