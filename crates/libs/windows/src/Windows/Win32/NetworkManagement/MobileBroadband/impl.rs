#[cfg(feature = "Win32_System_Com")]
pub trait IDummyMBNUCMExtImpl: Sized + IDispatchImpl {}
pub trait IMbnConnectionImpl: Sized {
    fn ConnectionID();
    fn InterfaceID();
    fn Connect();
    fn Disconnect();
    fn GetConnectionState();
    fn GetVoiceCallState();
    fn GetActivationNetworkError();
}
pub trait IMbnConnectionContextImpl: Sized {
    fn GetProvisionedContexts();
    fn SetProvisionedContext();
}
pub trait IMbnConnectionContextEventsImpl: Sized {
    fn OnProvisionedContextListChange();
    fn OnSetProvisionedContextComplete();
}
pub trait IMbnConnectionEventsImpl: Sized {
    fn OnConnectComplete();
    fn OnDisconnectComplete();
    fn OnConnectStateChange();
    fn OnVoiceCallStateChange();
}
pub trait IMbnConnectionManagerImpl: Sized {
    fn GetConnection();
    fn GetConnections();
}
pub trait IMbnConnectionManagerEventsImpl: Sized {
    fn OnConnectionArrival();
    fn OnConnectionRemoval();
}
pub trait IMbnConnectionProfileImpl: Sized {
    fn GetProfileXmlData();
    fn UpdateProfile();
    fn Delete();
}
pub trait IMbnConnectionProfileEventsImpl: Sized {
    fn OnProfileUpdate();
}
pub trait IMbnConnectionProfileManagerImpl: Sized {
    fn GetConnectionProfiles();
    fn GetConnectionProfile();
    fn CreateConnectionProfile();
}
pub trait IMbnConnectionProfileManagerEventsImpl: Sized {
    fn OnConnectionProfileArrival();
    fn OnConnectionProfileRemoval();
}
pub trait IMbnDeviceServiceImpl: Sized {
    fn QuerySupportedCommands();
    fn OpenCommandSession();
    fn CloseCommandSession();
    fn SetCommand();
    fn QueryCommand();
    fn OpenDataSession();
    fn CloseDataSession();
    fn WriteData();
    fn InterfaceID();
    fn DeviceServiceID();
    fn IsCommandSessionOpen();
    fn IsDataSessionOpen();
}
pub trait IMbnDeviceServiceStateEventsImpl: Sized {
    fn OnSessionsStateChange();
}
pub trait IMbnDeviceServicesContextImpl: Sized {
    fn EnumerateDeviceServices();
    fn GetDeviceService();
    fn MaxCommandSize();
    fn MaxDataSize();
}
pub trait IMbnDeviceServicesEventsImpl: Sized {
    fn OnQuerySupportedCommandsComplete();
    fn OnOpenCommandSessionComplete();
    fn OnCloseCommandSessionComplete();
    fn OnSetCommandComplete();
    fn OnQueryCommandComplete();
    fn OnEventNotification();
    fn OnOpenDataSessionComplete();
    fn OnCloseDataSessionComplete();
    fn OnWriteDataComplete();
    fn OnReadData();
    fn OnInterfaceStateChange();
}
pub trait IMbnDeviceServicesManagerImpl: Sized {
    fn GetDeviceServicesContext();
}
pub trait IMbnInterfaceImpl: Sized {
    fn InterfaceID();
    fn GetInterfaceCapability();
    fn GetSubscriberInformation();
    fn GetReadyState();
    fn InEmergencyMode();
    fn GetHomeProvider();
    fn GetPreferredProviders();
    fn SetPreferredProviders();
    fn GetVisibleProviders();
    fn ScanNetwork();
    fn GetConnection();
}
pub trait IMbnInterfaceEventsImpl: Sized {
    fn OnInterfaceCapabilityAvailable();
    fn OnSubscriberInformationChange();
    fn OnReadyStateChange();
    fn OnEmergencyModeChange();
    fn OnHomeProviderAvailable();
    fn OnPreferredProvidersChange();
    fn OnSetPreferredProvidersComplete();
    fn OnScanNetworkComplete();
}
pub trait IMbnInterfaceManagerImpl: Sized {
    fn GetInterface();
    fn GetInterfaces();
}
pub trait IMbnInterfaceManagerEventsImpl: Sized {
    fn OnInterfaceArrival();
    fn OnInterfaceRemoval();
}
pub trait IMbnMultiCarrierImpl: Sized {
    fn SetHomeProvider();
    fn GetPreferredProviders();
    fn GetVisibleProviders();
    fn GetSupportedCellularClasses();
    fn GetCurrentCellularClass();
    fn ScanNetwork();
}
pub trait IMbnMultiCarrierEventsImpl: Sized {
    fn OnSetHomeProviderComplete();
    fn OnCurrentCellularClassChange();
    fn OnPreferredProvidersChange();
    fn OnScanNetworkComplete();
    fn OnInterfaceCapabilityChange();
}
pub trait IMbnPinImpl: Sized {
    fn PinType();
    fn PinFormat();
    fn PinLengthMin();
    fn PinLengthMax();
    fn PinMode();
    fn Enable();
    fn Disable();
    fn Enter();
    fn Change();
    fn Unblock();
    fn GetPinManager();
}
pub trait IMbnPinEventsImpl: Sized {
    fn OnEnableComplete();
    fn OnDisableComplete();
    fn OnEnterComplete();
    fn OnChangeComplete();
    fn OnUnblockComplete();
}
pub trait IMbnPinManagerImpl: Sized {
    fn GetPinList();
    fn GetPin();
    fn GetPinState();
}
pub trait IMbnPinManagerEventsImpl: Sized {
    fn OnPinListAvailable();
    fn OnGetPinStateComplete();
}
pub trait IMbnRadioImpl: Sized {
    fn SoftwareRadioState();
    fn HardwareRadioState();
    fn SetSoftwareRadioState();
}
pub trait IMbnRadioEventsImpl: Sized {
    fn OnRadioStateChange();
    fn OnSetSoftwareRadioStateComplete();
}
pub trait IMbnRegistrationImpl: Sized {
    fn GetRegisterState();
    fn GetRegisterMode();
    fn GetProviderID();
    fn GetProviderName();
    fn GetRoamingText();
    fn GetAvailableDataClasses();
    fn GetCurrentDataClass();
    fn GetRegistrationNetworkError();
    fn GetPacketAttachNetworkError();
    fn SetRegisterMode();
}
pub trait IMbnRegistrationEventsImpl: Sized {
    fn OnRegisterModeAvailable();
    fn OnRegisterStateChange();
    fn OnPacketServiceStateChange();
    fn OnSetRegisterModeComplete();
}
pub trait IMbnServiceActivationImpl: Sized {
    fn Activate();
}
pub trait IMbnServiceActivationEventsImpl: Sized {
    fn OnActivationComplete();
}
pub trait IMbnSignalImpl: Sized {
    fn GetSignalStrength();
    fn GetSignalError();
}
pub trait IMbnSignalEventsImpl: Sized {
    fn OnSignalStateChange();
}
pub trait IMbnSmsImpl: Sized {
    fn GetSmsConfiguration();
    fn SetSmsConfiguration();
    fn SmsSendPdu();
    fn SmsSendCdma();
    fn SmsSendCdmaPdu();
    fn SmsRead();
    fn SmsDelete();
    fn GetSmsStatus();
}
pub trait IMbnSmsConfigurationImpl: Sized {
    fn ServiceCenterAddress();
    fn SetServiceCenterAddress();
    fn MaxMessageIndex();
    fn CdmaShortMsgSize();
    fn SmsFormat();
    fn SetSmsFormat();
}
pub trait IMbnSmsEventsImpl: Sized {
    fn OnSmsConfigurationChange();
    fn OnSetSmsConfigurationComplete();
    fn OnSmsSendComplete();
    fn OnSmsReadComplete();
    fn OnSmsNewClass0Message();
    fn OnSmsDeleteComplete();
    fn OnSmsStatusChange();
}
pub trait IMbnSmsReadMsgPduImpl: Sized {
    fn Index();
    fn Status();
    fn PduData();
    fn Message();
}
pub trait IMbnSmsReadMsgTextCdmaImpl: Sized {
    fn Index();
    fn Status();
    fn Address();
    fn Timestamp();
    fn EncodingID();
    fn LanguageID();
    fn SizeInCharacters();
    fn Message();
}
pub trait IMbnSubscriberInformationImpl: Sized {
    fn SubscriberID();
    fn SimIccID();
    fn TelephoneNumbers();
}
pub trait IMbnVendorSpecificEventsImpl: Sized {
    fn OnEventNotification();
    fn OnSetVendorSpecificComplete();
}
pub trait IMbnVendorSpecificOperationImpl: Sized {
    fn SetVendorSpecific();
}
