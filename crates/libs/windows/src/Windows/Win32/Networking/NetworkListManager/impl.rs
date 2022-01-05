#[cfg(feature = "Win32_System_Com")]
pub trait IEnumNetworkConnectionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumNetworksImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkImpl: Sized + IDispatchImpl {
    fn GetName();
    fn SetName();
    fn GetDescription();
    fn SetDescription();
    fn GetNetworkId();
    fn GetDomainType();
    fn GetNetworkConnections();
    fn GetTimeCreatedAndConnected();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn GetCategory();
    fn SetCategory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkConnectionImpl: Sized + IDispatchImpl {
    fn GetNetwork();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn GetConnectionId();
    fn GetAdapterId();
    fn GetDomainType();
}
pub trait INetworkConnectionCostImpl: Sized {
    fn GetCost();
    fn GetDataPlanStatus();
}
pub trait INetworkConnectionCostEventsImpl: Sized {
    fn ConnectionCostChanged();
    fn ConnectionDataPlanStatusChanged();
}
pub trait INetworkConnectionEventsImpl: Sized {
    fn NetworkConnectionConnectivityChanged();
    fn NetworkConnectionPropertyChanged();
}
pub trait INetworkCostManagerImpl: Sized {
    fn GetCost();
    fn GetDataPlanStatus();
    fn SetDestinationAddresses();
}
pub trait INetworkCostManagerEventsImpl: Sized {
    fn CostChanged();
    fn DataPlanStatusChanged();
}
pub trait INetworkEventsImpl: Sized {
    fn NetworkAdded();
    fn NetworkDeleted();
    fn NetworkConnectivityChanged();
    fn NetworkPropertyChanged();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetworkListManagerImpl: Sized + IDispatchImpl {
    fn GetNetworks();
    fn GetNetwork();
    fn GetNetworkConnections();
    fn GetNetworkConnection();
    fn IsConnectedToInternet();
    fn IsConnected();
    fn GetConnectivity();
    fn SetSimulatedProfileInfo();
    fn ClearSimulatedProfileInfo();
}
pub trait INetworkListManagerEventsImpl: Sized {
    fn ConnectivityChanged();
}
