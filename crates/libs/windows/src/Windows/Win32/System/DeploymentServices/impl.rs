#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCacheableImpl: Sized + IDispatchImpl {
    fn Dirty();
    fn Discard();
    fn Refresh();
    fn Commit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportClientImpl: Sized + IDispatchImpl {
    fn Session();
    fn Id();
    fn Name();
    fn MacAddress();
    fn IpAddress();
    fn PercentCompletion();
    fn JoinDuration();
    fn CpuUtilization();
    fn MemoryUtilization();
    fn NetworkUtilization();
    fn UserIdentity();
    fn Disconnect();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManagerImpl: Sized + IDispatchImpl {
    fn ServicePolicy();
    fn DiagnosticsPolicy();
    fn WdsTransportServicesRunning();
    fn EnableWdsTransportServices();
    fn DisableWdsTransportServices();
    fn StartWdsTransportServices();
    fn StopWdsTransportServices();
    fn RestartWdsTransportServices();
    fn NotifyWdsTransportServices();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportConfigurationManager2Impl: Sized + IWdsTransportConfigurationManagerImpl + IDispatchImpl {
    fn MulticastSessionPolicy();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContentImpl: Sized + IDispatchImpl {
    fn Namespace();
    fn Id();
    fn Name();
    fn RetrieveSessions();
    fn Terminate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportContentProviderImpl: Sized + IDispatchImpl {
    fn Name();
    fn Description();
    fn FilePath();
    fn InitializationRoutine();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportDiagnosticsPolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn Enabled();
    fn SetEnabled();
    fn Components();
    fn SetComponents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportManagerImpl: Sized + IDispatchImpl {
    fn GetWdsTransportServer();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportMulticastSessionPolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn SlowClientHandling();
    fn SetSlowClientHandling();
    fn AutoDisconnectThreshold();
    fn SetAutoDisconnectThreshold();
    fn MultistreamStreamCount();
    fn SetMultistreamStreamCount();
    fn SlowClientFallback();
    fn SetSlowClientFallback();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceImpl: Sized + IDispatchImpl {
    fn Type();
    fn Id();
    fn Name();
    fn SetName();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Description();
    fn SetDescription();
    fn ContentProvider();
    fn SetContentProvider();
    fn Configuration();
    fn SetConfiguration();
    fn Registered();
    fn Tombstoned();
    fn TombstoneTime();
    fn TransmissionStarted();
    fn Register();
    fn Deregister();
    fn Clone();
    fn Refresh();
    fn RetrieveContents();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceAutoCastImpl: Sized + IWdsTransportNamespaceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceManagerImpl: Sized + IDispatchImpl {
    fn CreateNamespace();
    fn RetrieveNamespace();
    fn RetrieveNamespaces();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastImpl: Sized + IWdsTransportNamespaceImpl + IDispatchImpl {
    fn StartTransmission();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastAutoStartImpl: Sized + IWdsTransportNamespaceScheduledCastImpl + IWdsTransportNamespaceImpl + IDispatchImpl {
    fn MinimumClients();
    fn SetMinimumClients();
    fn StartTime();
    fn SetStartTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportNamespaceScheduledCastManualStartImpl: Sized + IWdsTransportNamespaceScheduledCastImpl + IWdsTransportNamespaceImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServerImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetupManager();
    fn ConfigurationManager();
    fn NamespaceManager();
    fn DisconnectClient();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServer2Impl: Sized + IWdsTransportServerImpl + IDispatchImpl {
    fn TftpManager();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicyImpl: Sized + IWdsTransportCacheableImpl + IDispatchImpl {
    fn IpAddressSource();
    fn SetIpAddressSource();
    fn StartIpAddress();
    fn SetStartIpAddress();
    fn EndIpAddress();
    fn SetEndIpAddress();
    fn StartPort();
    fn SetStartPort();
    fn EndPort();
    fn SetEndPort();
    fn NetworkProfile();
    fn SetNetworkProfile();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportServicePolicy2Impl: Sized + IWdsTransportServicePolicyImpl + IWdsTransportCacheableImpl + IDispatchImpl {
    fn UdpPortPolicy();
    fn SetUdpPortPolicy();
    fn TftpMaximumBlockSize();
    fn SetTftpMaximumBlockSize();
    fn EnableTftpVariableWindowExtension();
    fn SetEnableTftpVariableWindowExtension();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSessionImpl: Sized + IDispatchImpl {
    fn Content();
    fn Id();
    fn NetworkInterfaceName();
    fn NetworkInterfaceAddress();
    fn TransferRate();
    fn MasterClientId();
    fn RetrieveClients();
    fn Terminate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManagerImpl: Sized + IDispatchImpl {
    fn Version();
    fn InstalledFeatures();
    fn Protocols();
    fn RegisterContentProvider();
    fn DeregisterContentProvider();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportSetupManager2Impl: Sized + IWdsTransportSetupManagerImpl + IDispatchImpl {
    fn TftpCapabilities();
    fn ContentProviders();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpClientImpl: Sized + IDispatchImpl {
    fn FileName();
    fn IpAddress();
    fn Timeout();
    fn CurrentFileOffset();
    fn FileSize();
    fn BlockSize();
    fn WindowSize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWdsTransportTftpManagerImpl: Sized + IDispatchImpl {
    fn RetrieveTftpClients();
}
