#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IEnumNetworkConnections();
    fn IEnumNetworks();
    fn INetwork();
    fn INetworkConnection();
    fn INetworkConnectionCost();
    fn INetworkConnectionCostEvents();
    fn INetworkConnectionEvents();
    fn INetworkCostManager();
    fn INetworkCostManagerEvents();
    fn INetworkEvents();
    fn INetworkListManager();
    fn INetworkListManagerEvents();
    fn NLM_CONNECTION_COST();
    fn NLM_CONNECTION_PROPERTY_CHANGE();
    fn NLM_CONNECTIVITY();
    fn NLM_DATAPLAN_STATUS();
    fn NLM_DOMAIN_TYPE();
    fn NLM_ENUM_NETWORK();
    fn NLM_INTERNET_CONNECTIVITY();
    fn NLM_MAX_ADDRESS_LIST_SIZE();
    fn NLM_NETWORK_CATEGORY();
    fn NLM_NETWORK_CLASS();
    fn NLM_NETWORK_PROPERTY_CHANGE();
    fn NLM_SIMULATED_PROFILE_INFO();
    fn NLM_SOCKADDR();
    fn NLM_UNKNOWN_DATAPLAN_STATUS();
    fn NLM_USAGE_DATA();
    fn NetworkListManager();
}
