#[cfg(feature = "implement_exclusive")]
pub trait IAttributedNetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn AttributionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AttributionThumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContextImpl: Sized {
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProviderId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessPointName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassword(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCompressionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCompressionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationType(&self) -> ::windows::core::Result<CellularApnAuthenticationType>;
    fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICellularApnContext2Impl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCostImpl: Sized {
    fn NetworkCostType(&self) -> ::windows::core::Result<NetworkCostType>;
    fn Roaming(&self) -> ::windows::core::Result<bool>;
    fn OverDataLimit(&self) -> ::windows::core::Result<bool>;
    fn ApproachingDataLimit(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionCost2Impl: Sized {
    fn BackgroundDataUsageRestricted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkConnectivityLevel(&self) -> ::windows::core::Result<NetworkConnectivityLevel>;
    fn GetNetworkNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetConnectionCost(&self) -> ::windows::core::Result<ConnectionCost>;
    fn GetDataPlanStatus(&self) -> ::windows::core::Result<DataPlanStatus>;
    fn NetworkAdapter(&self) -> ::windows::core::Result<NetworkAdapter>;
    fn GetLocalUsage(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime) -> ::windows::core::Result<DataUsage>;
    fn GetLocalUsagePerRoamingStates(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: RoamingStates) -> ::windows::core::Result<DataUsage>;
    fn NetworkSecuritySettings(&self) -> ::windows::core::Result<NetworkSecuritySettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile2Impl: Sized {
    fn IsWwanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn IsWlanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn WwanConnectionProfileDetails(&self) -> ::windows::core::Result<WwanConnectionProfileDetails>;
    fn WlanConnectionProfileDetails(&self) -> ::windows::core::Result<WlanConnectionProfileDetails>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
    fn GetSignalBars(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
    fn GetDomainConnectivityLevel(&self) -> ::windows::core::Result<DomainConnectivityLevel>;
    fn GetNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>>;
    fn GetConnectivityIntervalsAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile3Impl: Sized {
    fn GetAttributedNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile4Impl: Sized {
    fn GetProviderNetworkUsageAsync(&self, starttime: &super::super::Foundation::DateTime, endtime: &super::super::Foundation::DateTime, states: &NetworkUsageStates) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfile5Impl: Sized {
    fn CanDelete(&self) -> ::windows::core::Result<bool>;
    fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilterImpl: Sized {
    fn SetIsConnected(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnected(&self) -> ::windows::core::Result<bool>;
    fn SetIsWwanConnectionProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsWwanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn SetIsWlanConnectionProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsWlanConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn SetNetworkCostType(&self, value: NetworkCostType) -> ::windows::core::Result<()>;
    fn NetworkCostType(&self) -> ::windows::core::Result<NetworkCostType>;
    fn SetServiceProviderGuid(&self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilter2Impl: Sized {
    fn SetIsRoaming(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsRoaming(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsOverDataLimit(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsOverDataLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn SetIsBackgroundDataUsageRestricted(&self, value: &::core::option::Option<super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsBackgroundDataUsageRestricted(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>>;
    fn RawData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectionProfileFilter3Impl: Sized {
    fn SetPurposeGuid(&self, value: &::core::option::Option<super::super::Foundation::IReference<::windows::core::GUID>>) -> ::windows::core::Result<()>;
    fn PurposeGuid(&self) -> ::windows::core::Result<super::super::Foundation::IReference<::windows::core::GUID>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConnectionSessionImpl: Sized + IClosableImpl {
    fn ConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectivityIntervalImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ConnectionDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IConnectivityManagerStaticsImpl: Sized {
    fn AcquireConnectionAsync(&self, cellularapncontext: &::core::option::Option<CellularApnContext>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>>;
    fn AddHttpRoutePolicy(&self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
    fn RemoveHttpRoutePolicy(&self, routepolicy: &::core::option::Option<RoutePolicy>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPlanStatusImpl: Sized {
    fn DataPlanUsage(&self) -> ::windows::core::Result<DataPlanUsage>;
    fn DataLimitInMegabytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn InboundBitsPerSecond(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn OutboundBitsPerSecond(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn NextBillingCycle(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn MaxTransferSizeInMegabytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPlanUsageImpl: Sized {
    fn MegabytesUsed(&self) -> ::windows::core::Result<u32>;
    fn LastSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDataUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIPInformationImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<NetworkAdapter>;
    fn PrefixLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanIdentifierImpl: Sized {
    fn InfrastructureId(&self) -> ::windows::core::Result<LanIdentifierData>;
    fn PortId(&self) -> ::windows::core::Result<LanIdentifierData>;
    fn NetworkAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanIdentifierDataImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<u32>;
    fn Value(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u8>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkAdapterImpl: Sized {
    fn OutboundMaxBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn InboundMaxBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn IanaInterfaceType(&self) -> ::windows::core::Result<u32>;
    fn NetworkItem(&self) -> ::windows::core::Result<NetworkItem>;
    fn NetworkAdapterId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetConnectedProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkInformationStaticsImpl: Sized {
    fn GetConnectionProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>;
    fn GetInternetConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
    fn GetLanIdentifiers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>>;
    fn GetHostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn GetProxyConfigurationAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>;
    fn GetSortedEndpointPairs(&self, destinationlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::EndpointPair>>, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>;
    fn NetworkStatusChanged(&self, networkstatushandler: &::core::option::Option<NetworkStatusChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNetworkStatusChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkInformationStatics2Impl: Sized {
    fn FindConnectionProfilesAsync(&self, pprofilefilter: &::core::option::Option<ConnectionProfileFilter>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkItemImpl: Sized {
    fn NetworkId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetNetworkTypes(&self) -> ::windows::core::Result<NetworkTypes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkSecuritySettingsImpl: Sized {
    fn NetworkAuthenticationType(&self) -> ::windows::core::Result<NetworkAuthenticationType>;
    fn NetworkEncryptionType(&self) -> ::windows::core::Result<NetworkEncryptionType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetailsImpl: Sized {
    fn HasNewInternetConnectionProfile(&self) -> ::windows::core::Result<bool>;
    fn HasNewConnectionCost(&self) -> ::windows::core::Result<bool>;
    fn HasNewNetworkConnectivityLevel(&self) -> ::windows::core::Result<bool>;
    fn HasNewDomainConnectivityLevel(&self) -> ::windows::core::Result<bool>;
    fn HasNewHostNameList(&self) -> ::windows::core::Result<bool>;
    fn HasNewWwanRegistrationState(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkStateChangeEventDetails2Impl: Sized {
    fn HasNewTetheringOperationalState(&self) -> ::windows::core::Result<bool>;
    fn HasNewTetheringClientCount(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn ConnectionDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderNetworkUsageImpl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u64>;
    fn BytesReceived(&self) -> ::windows::core::Result<u64>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProxyConfigurationImpl: Sized {
    fn ProxyUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
    fn CanConnectDirectly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyImpl: Sized {
    fn ConnectionProfile(&self) -> ::windows::core::Result<ConnectionProfile>;
    fn HostName(&self) -> ::windows::core::Result<super::HostName>;
    fn HostNameType(&self) -> ::windows::core::Result<super::DomainNameType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutePolicyFactoryImpl: Sized {
    fn CreateRoutePolicy(&self, connectionprofile: &::core::option::Option<ConnectionProfile>, hostname: &::core::option::Option<super::HostName>, r#type: super::DomainNameType) -> ::windows::core::Result<RoutePolicy>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWlanConnectionProfileDetailsImpl: Sized {
    fn GetConnectedSsid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwanConnectionProfileDetailsImpl: Sized {
    fn HomeProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNetworkRegistrationState(&self) -> ::windows::core::Result<WwanNetworkRegistrationState>;
    fn GetCurrentDataClass(&self) -> ::windows::core::Result<WwanDataClass>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwanConnectionProfileDetails2Impl: Sized {
    fn IPKind(&self) -> ::windows::core::Result<WwanNetworkIPKind>;
    fn PurposeGuids(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
}
