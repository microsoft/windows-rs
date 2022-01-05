#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicPortMappingImpl: Sized + IDispatchImpl {
    fn ExternalIPAddress();
    fn RemoteHost();
    fn ExternalPort();
    fn Protocol();
    fn InternalPort();
    fn InternalClient();
    fn Enabled();
    fn Description();
    fn LeaseDuration();
    fn RenewLease();
    fn EditInternalClient();
    fn Enable();
    fn EditDescription();
    fn EditInternalPort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
pub trait IEnumNetConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetSharingEveryConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetSharingPortMappingImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetSharingPrivateConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumNetSharingPublicConnectionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INATEventManagerImpl: Sized + IDispatchImpl {
    fn SetExternalIPAddressCallback();
    fn SetNumberOfEntriesCallback();
}
pub trait INATExternalIPAddressCallbackImpl: Sized {
    fn NewExternalIPAddress();
}
pub trait INATNumberOfEntriesCallbackImpl: Sized {
    fn NewNumberOfEntries();
}
pub trait INetConnectionImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Delete();
    fn Duplicate();
    fn GetProperties();
    fn GetUiObjectClassId();
    fn Rename();
}
pub trait INetConnectionConnectUiImpl: Sized {
    fn SetConnection();
    fn Connect();
    fn Disconnect();
}
pub trait INetConnectionManagerImpl: Sized {
    fn EnumConnections();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetConnectionPropsImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Name();
    fn DeviceName();
    fn Status();
    fn MediaType();
    fn Characteristics();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwAuthorizedApplicationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn ProcessImageFileName();
    fn SetProcessImageFileName();
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwAuthorizedApplicationsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwIcmpSettingsImpl: Sized + IDispatchImpl {
    fn AllowOutboundDestinationUnreachable();
    fn SetAllowOutboundDestinationUnreachable();
    fn AllowRedirect();
    fn SetAllowRedirect();
    fn AllowInboundEchoRequest();
    fn SetAllowInboundEchoRequest();
    fn AllowOutboundTimeExceeded();
    fn SetAllowOutboundTimeExceeded();
    fn AllowOutboundParameterProblem();
    fn SetAllowOutboundParameterProblem();
    fn AllowOutboundSourceQuench();
    fn SetAllowOutboundSourceQuench();
    fn AllowInboundRouterRequest();
    fn SetAllowInboundRouterRequest();
    fn AllowInboundTimestampRequest();
    fn SetAllowInboundTimestampRequest();
    fn AllowInboundMaskRequest();
    fn SetAllowInboundMaskRequest();
    fn AllowOutboundPacketTooBig();
    fn SetAllowOutboundPacketTooBig();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwMgrImpl: Sized + IDispatchImpl {
    fn LocalPolicy();
    fn CurrentProfileType();
    fn RestoreDefaults();
    fn IsPortAllowed();
    fn IsIcmpTypeAllowed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwOpenPortImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn IpVersion();
    fn SetIpVersion();
    fn Protocol();
    fn SetProtocol();
    fn Port();
    fn SetPort();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
    fn BuiltIn();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwOpenPortsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwPolicyImpl: Sized + IDispatchImpl {
    fn CurrentProfile();
    fn GetProfileByType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwPolicy2Impl: Sized + IDispatchImpl {
    fn CurrentProfileTypes();
    fn FirewallEnabled();
    fn SetFirewallEnabled();
    fn ExcludedInterfaces();
    fn SetExcludedInterfaces();
    fn BlockAllInboundTraffic();
    fn SetBlockAllInboundTraffic();
    fn NotificationsDisabled();
    fn SetNotificationsDisabled();
    fn UnicastResponsesToMulticastBroadcastDisabled();
    fn SetUnicastResponsesToMulticastBroadcastDisabled();
    fn Rules();
    fn ServiceRestriction();
    fn EnableRuleGroup();
    fn IsRuleGroupEnabled();
    fn RestoreLocalFirewallDefaults();
    fn DefaultInboundAction();
    fn SetDefaultInboundAction();
    fn DefaultOutboundAction();
    fn SetDefaultOutboundAction();
    fn IsRuleGroupCurrentlyEnabled();
    fn LocalPolicyModifyState();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProductImpl: Sized + IDispatchImpl {
    fn RuleCategories();
    fn SetRuleCategories();
    fn DisplayName();
    fn SetDisplayName();
    fn PathToSignedProductExe();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProductsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Register();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProfileImpl: Sized + IDispatchImpl {
    fn Type();
    fn FirewallEnabled();
    fn SetFirewallEnabled();
    fn ExceptionsNotAllowed();
    fn SetExceptionsNotAllowed();
    fn NotificationsDisabled();
    fn SetNotificationsDisabled();
    fn UnicastResponsesToMulticastBroadcastDisabled();
    fn SetUnicastResponsesToMulticastBroadcastDisabled();
    fn RemoteAdminSettings();
    fn IcmpSettings();
    fn GloballyOpenPorts();
    fn Services();
    fn AuthorizedApplications();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRemoteAdminSettingsImpl: Sized + IDispatchImpl {
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRuleImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationName();
    fn SetApplicationName();
    fn ServiceName();
    fn SetServiceName();
    fn Protocol();
    fn SetProtocol();
    fn LocalPorts();
    fn SetLocalPorts();
    fn RemotePorts();
    fn SetRemotePorts();
    fn LocalAddresses();
    fn SetLocalAddresses();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn IcmpTypesAndCodes();
    fn SetIcmpTypesAndCodes();
    fn Direction();
    fn SetDirection();
    fn Interfaces();
    fn SetInterfaces();
    fn InterfaceTypes();
    fn SetInterfaceTypes();
    fn Enabled();
    fn SetEnabled();
    fn Grouping();
    fn SetGrouping();
    fn Profiles();
    fn SetProfiles();
    fn EdgeTraversal();
    fn SetEdgeTraversal();
    fn Action();
    fn SetAction();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule2Impl: Sized + INetFwRuleImpl + IDispatchImpl {
    fn EdgeTraversalOptions();
    fn SetEdgeTraversalOptions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule3Impl: Sized + INetFwRule2Impl + INetFwRuleImpl + IDispatchImpl {
    fn LocalAppPackageId();
    fn SetLocalAppPackageId();
    fn LocalUserOwner();
    fn SetLocalUserOwner();
    fn LocalUserAuthorizedList();
    fn SetLocalUserAuthorizedList();
    fn RemoteUserAuthorizedList();
    fn SetRemoteUserAuthorizedList();
    fn RemoteMachineAuthorizedList();
    fn SetRemoteMachineAuthorizedList();
    fn SecureFlags();
    fn SetSecureFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRulesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServiceImpl: Sized + IDispatchImpl {
    fn Name();
    fn Type();
    fn Customized();
    fn IpVersion();
    fn SetIpVersion();
    fn Scope();
    fn SetScope();
    fn RemoteAddresses();
    fn SetRemoteAddresses();
    fn Enabled();
    fn SetEnabled();
    fn GloballyOpenPorts();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServiceRestrictionImpl: Sized + IDispatchImpl {
    fn RestrictService();
    fn ServiceRestricted();
    fn Rules();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServicesImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingConfigurationImpl: Sized + IDispatchImpl {
    fn SharingEnabled();
    fn SharingConnectionType();
    fn DisableSharing();
    fn EnableSharing();
    fn InternetFirewallEnabled();
    fn DisableInternetFirewall();
    fn EnableInternetFirewall();
    fn EnumPortMappings();
    fn AddPortMapping();
    fn RemovePortMapping();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingEveryConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingManagerImpl: Sized + IDispatchImpl {
    fn SharingInstalled();
    fn EnumPublicConnections();
    fn EnumPrivateConnections();
    fn INetSharingConfigurationForINetConnection();
    fn EnumEveryConnection();
    fn NetConnectionProps();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingImpl: Sized + IDispatchImpl {
    fn Disable();
    fn Enable();
    fn Properties();
    fn Delete();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingPropsImpl: Sized + IDispatchImpl {
    fn Name();
    fn IPProtocol();
    fn ExternalPort();
    fn InternalPort();
    fn Options();
    fn TargetName();
    fn TargetIPAddress();
    fn Enabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPrivateConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPublicConnectionCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStaticPortMappingImpl: Sized + IDispatchImpl {
    fn ExternalIPAddress();
    fn ExternalPort();
    fn InternalPort();
    fn Protocol();
    fn InternalClient();
    fn Enabled();
    fn Description();
    fn EditInternalClient();
    fn Enable();
    fn EditDescription();
    fn EditInternalPort();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStaticPortMappingCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPNATImpl: Sized + IDispatchImpl {
    fn StaticPortMappingCollection();
    fn DynamicPortMappingCollection();
    fn NATEventManager();
}
