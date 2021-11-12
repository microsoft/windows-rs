#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IEnumNetworkConnections(i32);
pub struct IEnumNetworks(i32);
pub struct INetwork(i32);
pub struct INetworkConnection(i32);
pub struct INetworkConnectionCost(i32);
pub struct INetworkConnectionCostEvents(i32);
pub struct INetworkConnectionEvents(i32);
pub struct INetworkCostManager(i32);
pub struct INetworkCostManagerEvents(i32);
pub struct INetworkEvents(i32);
pub struct INetworkListManager(i32);
pub struct INetworkListManagerEvents(i32);
pub struct NLM_CONNECTION_COST(i32);
pub struct NLM_CONNECTION_PROPERTY_CHANGE(i32);
pub struct NLM_CONNECTIVITY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_DATAPLAN_STATUS(i32);
pub struct NLM_DOMAIN_TYPE(i32);
pub struct NLM_ENUM_NETWORK(i32);
pub struct NLM_INTERNET_CONNECTIVITY(i32);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub const NLM_MAX_ADDRESS_LIST_SIZE: u32 = 10u32;
pub struct NLM_NETWORK_CATEGORY(i32);
pub struct NLM_NETWORK_CLASS(i32);
pub struct NLM_NETWORK_PROPERTY_CHANGE(i32);
pub struct NLM_SIMULATED_PROFILE_INFO(i32);
pub struct NLM_SOCKADDR(i32);
#[doc = "*Required features: `Win32_Networking_NetworkListManager`*"]
pub const NLM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NLM_USAGE_DATA(i32);
pub struct NetworkListManager(i32);
