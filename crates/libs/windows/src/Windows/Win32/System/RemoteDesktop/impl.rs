#[cfg(feature = "Win32_System_Com")]
pub trait IADsTSUserExImpl: Sized + IDispatchImpl {
    fn TerminalServicesProfilePath();
    fn SetTerminalServicesProfilePath();
    fn TerminalServicesHomeDirectory();
    fn SetTerminalServicesHomeDirectory();
    fn TerminalServicesHomeDrive();
    fn SetTerminalServicesHomeDrive();
    fn AllowLogon();
    fn SetAllowLogon();
    fn EnableRemoteControl();
    fn SetEnableRemoteControl();
    fn MaxDisconnectionTime();
    fn SetMaxDisconnectionTime();
    fn MaxConnectionTime();
    fn SetMaxConnectionTime();
    fn MaxIdleTime();
    fn SetMaxIdleTime();
    fn ReconnectionAction();
    fn SetReconnectionAction();
    fn BrokenConnectionAction();
    fn SetBrokenConnectionAction();
    fn ConnectClientDrivesAtLogon();
    fn SetConnectClientDrivesAtLogon();
    fn ConnectClientPrintersAtLogon();
    fn SetConnectClientPrintersAtLogon();
    fn DefaultToMainPrinter();
    fn SetDefaultToMainPrinter();
    fn TerminalServicesWorkDirectory();
    fn SetTerminalServicesWorkDirectory();
    fn TerminalServicesInitialProgram();
    fn SetTerminalServicesInitialProgram();
}
pub trait IAudioDeviceEndpointImpl: Sized {
    fn SetBuffer();
    fn GetRTCaps();
    fn GetEventDrivenCapable();
    fn WriteExclusiveModeParametersToSharedMemory();
}
pub trait IAudioEndpointImpl: Sized {
    fn GetFrameFormat();
    fn GetFramesPerPacket();
    fn GetLatency();
    fn SetStreamFlags();
    fn SetEventHandle();
}
pub trait IAudioEndpointControlImpl: Sized {
    fn Start();
    fn Reset();
    fn Stop();
}
pub trait IAudioEndpointRTImpl: Sized {
    fn GetCurrentPadding();
    fn ProcessingComplete();
    fn SetPinInactive();
    fn SetPinActive();
}
pub trait IAudioInputEndpointRTImpl: Sized {
    fn GetInputDataPointer();
    fn ReleaseInputDataPointer();
    fn PulseEndpoint();
}
pub trait IAudioOutputEndpointRTImpl: Sized {
    fn GetOutputDataPointer();
    fn ReleaseOutputDataPointer();
    fn PulseEndpoint();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Disconnect();
    fn Reconnect();
    fn Settings();
    fn Actions();
    fn TouchPointer();
    fn DeleteSavedCredentials();
    fn UpdateSessionDisplaySettings();
    fn attachEvent();
    fn detachEvent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientActionsImpl: Sized + IDispatchImpl {
    fn SuspendScreenUpdates();
    fn ResumeScreenUpdates();
    fn ExecuteRemoteAction();
    fn GetSnapshot();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientSettingsImpl: Sized + IDispatchImpl {
    fn ApplySettings();
    fn RetrieveSettings();
    fn GetRdpProperty();
    fn SetRdpProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRemoteDesktopClientTouchPointerImpl: Sized + IDispatchImpl {
    fn SetEnabled();
    fn Enabled();
    fn SetEventsEnabled();
    fn EventsEnabled();
    fn SetPointerSpeed();
    fn PointerSpeed();
}
pub trait IRemoteSystemAdditionalInfoProviderImpl: Sized {
    fn GetAdditionalInfo();
}
pub trait ITSGAccountingEngineImpl: Sized {
    fn DoAccounting();
}
pub trait ITSGAuthenticateUserSinkImpl: Sized {
    fn OnUserAuthenticated();
    fn OnUserAuthenticationFailed();
    fn ReauthenticateUser();
    fn DisconnectUser();
}
pub trait ITSGAuthenticationEngineImpl: Sized {
    fn AuthenticateUser();
    fn CancelAuthentication();
}
pub trait ITSGAuthorizeConnectionSinkImpl: Sized {
    fn OnConnectionAuthorized();
}
pub trait ITSGAuthorizeResourceSinkImpl: Sized {
    fn OnChannelAuthorized();
}
pub trait ITSGPolicyEngineImpl: Sized {
    fn AuthorizeConnection();
    fn AuthorizeResource();
    fn Refresh();
    fn IsQuarantineEnabled();
}
pub trait ITsSbBaseNotifySinkImpl: Sized {
    fn OnError();
    fn OnReportStatus();
}
pub trait ITsSbClientConnectionImpl: Sized {
    fn UserName();
    fn Domain();
    fn InitialProgram();
    fn LoadBalanceResult();
    fn FarmName();
    fn PutContext();
    fn GetContext();
    fn Environment();
    fn ConnectionError();
    fn SamUserAccount();
    fn ClientConnectionPropertySet();
    fn IsFirstAssignment();
    fn RdFarmType();
    fn UserSidString();
    fn GetDisconnectedSession();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbClientConnectionPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
pub trait ITsSbEnvironmentImpl: Sized {
    fn Name();
    fn ServerWeight();
    fn EnvironmentPropertySet();
    fn SetEnvironmentPropertySet();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironmentPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
pub trait ITsSbFilterPluginStoreImpl: Sized {
    fn SaveProperties();
    fn EnumerateProperties();
    fn DeleteProperties();
}
pub trait ITsSbGenericNotifySinkImpl: Sized {
    fn OnCompleted();
    fn GetWaitTimeout();
}
pub trait ITsSbGlobalStoreImpl: Sized {
    fn QueryTarget();
    fn QuerySessionBySessionId();
    fn EnumerateFarms();
    fn EnumerateTargets();
    fn EnumerateEnvironmentsByProvider();
    fn EnumerateSessions();
    fn GetFarmProperty();
}
pub trait ITsSbLoadBalanceResultImpl: Sized {
    fn TargetName();
}
pub trait ITsSbLoadBalancingImpl: Sized + ITsSbPluginImpl {
    fn GetMostSuitableTarget();
}
pub trait ITsSbLoadBalancingNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnGetMostSuitableTarget();
}
pub trait ITsSbOrchestrationImpl: Sized + ITsSbPluginImpl {
    fn PrepareTargetForConnect();
}
pub trait ITsSbOrchestrationNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnReadyToConnect();
}
pub trait ITsSbPlacementImpl: Sized + ITsSbPluginImpl {
    fn QueryEnvironmentForTarget();
}
pub trait ITsSbPlacementNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnQueryEnvironmentCompleted();
}
pub trait ITsSbPluginImpl: Sized {
    fn Initialize();
    fn Terminate();
}
pub trait ITsSbPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnInitialized();
    fn OnTerminated();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPluginPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPropertySetImpl: Sized + IPropertyBagImpl {}
pub trait ITsSbProviderImpl: Sized {
    fn CreateTargetObject();
    fn CreateLoadBalanceResultObject();
    fn CreateSessionObject();
    fn CreatePluginPropertySet();
    fn CreateTargetPropertySetObject();
    fn CreateEnvironmentObject();
    fn GetResourcePluginStore();
    fn GetFilterPluginStore();
    fn RegisterForNotification();
    fn UnRegisterForNotification();
    fn GetInstanceOfGlobalStore();
    fn CreateEnvironmentPropertySetObject();
}
pub trait ITsSbProvisioningImpl: Sized + ITsSbPluginImpl {
    fn CreateVirtualMachines();
    fn PatchVirtualMachines();
    fn DeleteVirtualMachines();
    fn CancelJob();
}
pub trait ITsSbProvisioningPluginNotifySinkImpl: Sized {
    fn OnJobCreated();
    fn OnVirtualMachineStatusChanged();
    fn OnJobCompleted();
    fn OnJobCancelled();
    fn LockVirtualMachine();
    fn OnVirtualMachineHostStatusChanged();
}
pub trait ITsSbResourceNotificationImpl: Sized {
    fn NotifySessionChange();
    fn NotifyTargetChange();
    fn NotifyClientConnectionStateChange();
}
pub trait ITsSbResourceNotificationExImpl: Sized {
    fn NotifySessionChangeEx();
    fn NotifyTargetChangeEx();
    fn NotifyClientConnectionStateChangeEx();
}
pub trait ITsSbResourcePluginImpl: Sized + ITsSbPluginImpl {}
pub trait ITsSbResourcePluginStoreImpl: Sized {
    fn QueryTarget();
    fn QuerySessionBySessionId();
    fn AddTargetToStore();
    fn AddSessionToStore();
    fn AddEnvironmentToStore();
    fn RemoveEnvironmentFromStore();
    fn EnumerateFarms();
    fn QueryEnvironment();
    fn EnumerateEnvironments();
    fn SaveTarget();
    fn SaveEnvironment();
    fn SaveSession();
    fn SetTargetProperty();
    fn SetEnvironmentProperty();
    fn SetTargetState();
    fn SetSessionState();
    fn EnumerateTargets();
    fn EnumerateSessions();
    fn GetFarmProperty();
    fn DeleteTarget();
    fn SetTargetPropertyWithVersionCheck();
    fn SetEnvironmentPropertyWithVersionCheck();
    fn AcquireTargetLock();
    fn ReleaseTargetLock();
    fn TestAndSetServerState();
    fn SetServerWaitingToStart();
    fn GetServerState();
    fn SetServerDrainMode();
}
pub trait ITsSbServiceNotificationImpl: Sized {
    fn NotifyServiceFailure();
    fn NotifyServiceSuccess();
}
pub trait ITsSbSessionImpl: Sized {
    fn SessionId();
    fn TargetName();
    fn SetTargetName();
    fn Username();
    fn Domain();
    fn State();
    fn SetState();
    fn CreateTime();
    fn SetCreateTime();
    fn DisconnectTime();
    fn SetDisconnectTime();
    fn InitialProgram();
    fn SetInitialProgram();
    fn ClientDisplay();
    fn SetClientDisplay();
    fn ProtocolType();
    fn SetProtocolType();
}
pub trait ITsSbTargetImpl: Sized {
    fn TargetName();
    fn SetTargetName();
    fn FarmName();
    fn SetFarmName();
    fn TargetFQDN();
    fn SetTargetFQDN();
    fn TargetNetbios();
    fn SetTargetNetbios();
    fn IpAddresses();
    fn SetIpAddresses();
    fn TargetState();
    fn SetTargetState();
    fn TargetPropertySet();
    fn SetTargetPropertySet();
    fn EnvironmentName();
    fn SetEnvironmentName();
    fn NumSessions();
    fn NumPendingConnections();
    fn TargetLoad();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTargetPropertySetImpl: Sized + ITsSbPropertySetImpl + IPropertyBagImpl {}
pub trait ITsSbTaskInfoImpl: Sized {
    fn TargetId();
    fn StartTime();
    fn EndTime();
    fn Deadline();
    fn Identifier();
    fn Label();
    fn Context();
    fn Plugin();
    fn Status();
}
pub trait ITsSbTaskPluginImpl: Sized + ITsSbPluginImpl {
    fn InitializeTaskPlugin();
    fn SetTaskQueue();
}
pub trait ITsSbTaskPluginNotifySinkImpl: Sized + ITsSbBaseNotifySinkImpl {
    fn OnSetTaskTime();
    fn OnDeleteTaskTime();
    fn OnUpdateTaskStatus();
    fn OnReportTasks();
}
pub trait IWRdsEnhancedFastReconnectArbitratorImpl: Sized {
    fn GetSessionForEnhancedFastReconnect();
}
pub trait IWRdsGraphicsChannelImpl: Sized {
    fn Write();
    fn Close();
    fn Open();
}
pub trait IWRdsGraphicsChannelEventsImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
    fn OnChannelOpened();
    fn OnDataSent();
    fn OnMetricsUpdate();
}
pub trait IWRdsGraphicsChannelManagerImpl: Sized {
    fn CreateChannel();
}
pub trait IWRdsProtocolConnectionImpl: Sized {
    fn GetLogonErrorRedirector();
    fn AcceptConnection();
    fn GetClientData();
    fn GetClientMonitorData();
    fn GetUserCredentials();
    fn GetLicenseConnection();
    fn AuthenticateClientToSession();
    fn NotifySessionId();
    fn GetInputHandles();
    fn GetVideoHandle();
    fn ConnectNotify();
    fn IsUserAllowedToLogon();
    fn SessionArbitrationEnumeration();
    fn LogonNotify();
    fn PreDisconnect();
    fn DisconnectNotify();
    fn Close();
    fn GetProtocolStatus();
    fn GetLastInputTime();
    fn SetErrorInfo();
    fn CreateVirtualChannel();
    fn QueryProperty();
    fn GetShadowConnection();
    fn NotifyCommandProcessCreated();
}
pub trait IWRdsProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn GetConnectionId();
}
pub trait IWRdsProtocolConnectionSettingsImpl: Sized {
    fn SetConnectionSetting();
    fn GetConnectionSetting();
}
pub trait IWRdsProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
pub trait IWRdsProtocolListenerImpl: Sized {
    fn GetSettings();
    fn StartListen();
    fn StopListen();
}
pub trait IWRdsProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
pub trait IWRdsProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
pub trait IWRdsProtocolManagerImpl: Sized {
    fn Initialize();
    fn CreateListener();
    fn NotifyServiceStateChange();
    fn NotifySessionOfServiceStart();
    fn NotifySessionOfServiceStop();
    fn NotifySessionStateChange();
    fn NotifySettingsChange();
    fn Uninitialize();
}
pub trait IWRdsProtocolSettingsImpl: Sized {
    fn GetSettings();
    fn MergeSettings();
}
pub trait IWRdsProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
pub trait IWRdsProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
pub trait IWRdsWddmIddPropsImpl: Sized {
    fn GetHardwareId();
    fn OnDriverLoad();
    fn OnDriverUnload();
    fn EnableWddmIdd();
}
pub trait IWTSBitmapRenderServiceImpl: Sized {
    fn GetMappedRenderer();
}
pub trait IWTSBitmapRendererImpl: Sized {
    fn Render();
    fn GetRendererStatistics();
    fn RemoveMapping();
}
pub trait IWTSBitmapRendererCallbackImpl: Sized {
    fn OnTargetSizeChanged();
}
pub trait IWTSListenerImpl: Sized {
    fn GetConfiguration();
}
pub trait IWTSListenerCallbackImpl: Sized {
    fn OnNewChannelConnection();
}
pub trait IWTSPluginImpl: Sized {
    fn Initialize();
    fn Connected();
    fn Disconnected();
    fn Terminated();
}
pub trait IWTSPluginServiceProviderImpl: Sized {
    fn GetService();
}
pub trait IWTSProtocolConnectionImpl: Sized {
    fn GetLogonErrorRedirector();
    fn SendPolicyData();
    fn AcceptConnection();
    fn GetClientData();
    fn GetUserCredentials();
    fn GetLicenseConnection();
    fn AuthenticateClientToSession();
    fn NotifySessionId();
    fn GetProtocolHandles();
    fn ConnectNotify();
    fn IsUserAllowedToLogon();
    fn SessionArbitrationEnumeration();
    fn LogonNotify();
    fn GetUserData();
    fn DisconnectNotify();
    fn Close();
    fn GetProtocolStatus();
    fn GetLastInputTime();
    fn SetErrorInfo();
    fn SendBeep();
    fn CreateVirtualChannel();
    fn QueryProperty();
    fn GetShadowConnection();
}
pub trait IWTSProtocolConnectionCallbackImpl: Sized {
    fn OnReady();
    fn BrokenConnection();
    fn StopScreenUpdates();
    fn RedrawWindow();
    fn DisplayIOCtl();
}
pub trait IWTSProtocolLicenseConnectionImpl: Sized {
    fn RequestLicensingCapabilities();
    fn SendClientLicense();
    fn RequestClientLicense();
    fn ProtocolComplete();
}
pub trait IWTSProtocolListenerImpl: Sized {
    fn StartListen();
    fn StopListen();
}
pub trait IWTSProtocolListenerCallbackImpl: Sized {
    fn OnConnected();
}
pub trait IWTSProtocolLogonErrorRedirectorImpl: Sized {
    fn OnBeginPainting();
    fn RedirectStatus();
    fn RedirectMessage();
    fn RedirectLogonError();
}
pub trait IWTSProtocolManagerImpl: Sized {
    fn CreateListener();
    fn NotifyServiceStateChange();
    fn NotifySessionOfServiceStart();
    fn NotifySessionOfServiceStop();
    fn NotifySessionStateChange();
}
pub trait IWTSProtocolShadowCallbackImpl: Sized {
    fn StopShadow();
    fn InvokeTargetShadow();
}
pub trait IWTSProtocolShadowConnectionImpl: Sized {
    fn Start();
    fn Stop();
    fn DoTarget();
}
pub trait IWTSSBPluginImpl: Sized {
    fn Initialize();
    fn WTSSBX_MachineChangeNotification();
    fn WTSSBX_SessionChangeNotification();
    fn WTSSBX_GetMostSuitableServer();
    fn Terminated();
    fn WTSSBX_GetUserExternalSession();
}
pub trait IWTSVirtualChannelImpl: Sized {
    fn Write();
    fn Close();
}
pub trait IWTSVirtualChannelCallbackImpl: Sized {
    fn OnDataReceived();
    fn OnClose();
}
pub trait IWTSVirtualChannelManagerImpl: Sized {
    fn CreateListener();
}
pub trait IWorkspaceImpl: Sized {
    fn GetWorkspaceNames();
    fn StartRemoteApplication();
    fn GetProcessId();
}
pub trait IWorkspace2Impl: Sized + IWorkspaceImpl {
    fn StartRemoteApplicationEx();
}
pub trait IWorkspace3Impl: Sized + IWorkspace2Impl + IWorkspaceImpl {
    fn GetClaimsToken2();
    fn SetClaimsToken();
}
pub trait IWorkspaceClientExtImpl: Sized {
    fn GetResourceId();
    fn GetResourceDisplayName();
    fn IssueDisconnect();
}
pub trait IWorkspaceRegistrationImpl: Sized {
    fn AddResource();
    fn RemoveResource();
}
pub trait IWorkspaceRegistration2Impl: Sized + IWorkspaceRegistrationImpl {
    fn AddResourceEx();
    fn RemoveResourceEx();
}
pub trait IWorkspaceReportMessageImpl: Sized {
    fn RegisterErrorLogMessage();
    fn IsErrorMessageRegistered();
    fn RegisterErrorEvent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceResTypeRegistryImpl: Sized + IDispatchImpl {
    fn AddResourceType();
    fn DeleteResourceType();
    fn GetRegisteredFileExtensions();
    fn GetResourceTypeInfo();
    fn ModifyResourceType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptableImpl: Sized + IDispatchImpl {
    fn DisconnectWorkspace();
    fn StartWorkspace();
    fn IsWorkspaceCredentialSpecified();
    fn IsWorkspaceSSOEnabled();
    fn ClearWorkspaceCredential();
    fn OnAuthenticated();
    fn DisconnectWorkspaceByFriendlyName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable2Impl: Sized + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx();
    fn ResourceDismissed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspaceScriptable3Impl: Sized + IWorkspaceScriptable2Impl + IWorkspaceScriptableImpl + IDispatchImpl {
    fn StartWorkspaceEx2();
}
pub trait ItsPubPluginImpl: Sized {
    fn GetResourceList();
    fn GetResource();
    fn GetCacheLastUpdateTime();
    fn pluginName();
    fn pluginVersion();
    fn ResolveResource();
}
pub trait ItsPubPlugin2Impl: Sized + ItsPubPluginImpl {
    fn GetResource2List();
    fn GetResource2();
    fn ResolvePersonalDesktop();
    fn DeletePersonalDesktopAssignment();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ITSWkspEventsImpl: Sized + IDispatchImpl {}
