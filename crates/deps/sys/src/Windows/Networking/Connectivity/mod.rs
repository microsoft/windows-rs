#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AttributedNetworkUsage(pub *mut ::core::ffi::c_void);
pub struct CellularApnAuthenticationType(i32);
#[repr(transparent)]
pub struct CellularApnContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionCost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionProfile(pub *mut ::core::ffi::c_void);
pub struct ConnectionProfileDeleteStatus(i32);
#[repr(transparent)]
pub struct ConnectionProfileFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectionSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectivityInterval(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConnectivityManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPlanStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPlanUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataUsage(pub *mut ::core::ffi::c_void);
pub struct DataUsageGranularity(i32);
pub struct DomainConnectivityLevel(i32);
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
pub struct NetworkAuthenticationType(i32);
pub struct NetworkConnectivityLevel(i32);
pub struct NetworkCostType(i32);
pub struct NetworkEncryptionType(i32);
#[repr(transparent)]
pub struct NetworkInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkSecuritySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkStateChangeEventDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkStatusChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct NetworkTypes(i32);
#[repr(transparent)]
pub struct NetworkUsage(pub *mut ::core::ffi::c_void);
pub struct NetworkUsageStates(i32);
#[repr(transparent)]
pub struct ProviderNetworkUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProxyConfiguration(pub *mut ::core::ffi::c_void);
pub struct RoamingStates(i32);
#[repr(transparent)]
pub struct RoutePolicy(pub *mut ::core::ffi::c_void);
pub struct TriStates(i32);
#[repr(transparent)]
pub struct WlanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WwanConnectionProfileDetails(pub *mut ::core::ffi::c_void);
pub struct WwanContract(i32);
pub struct WwanDataClass(i32);
pub struct WwanNetworkIPKind(i32);
pub struct WwanNetworkRegistrationState(i32);
