pub trait IDot11AdHocInterfaceImpl: Sized {
    fn GetDeviceSignature();
    fn GetFriendlyName();
    fn IsDot11d();
    fn IsAdHocCapable();
    fn IsRadioOn();
    fn GetActiveNetwork();
    fn GetIEnumSecuritySettings();
    fn GetIEnumDot11AdHocNetworks();
    fn GetStatus();
}
pub trait IDot11AdHocInterfaceNotificationSinkImpl: Sized {
    fn OnConnectionStatusChange();
}
pub trait IDot11AdHocManagerImpl: Sized {
    fn CreateNetwork();
    fn CommitCreatedNetwork();
    fn GetIEnumDot11AdHocNetworks();
    fn GetIEnumDot11AdHocInterfaces();
    fn GetNetwork();
}
pub trait IDot11AdHocManagerNotificationSinkImpl: Sized {
    fn OnNetworkAdd();
    fn OnNetworkRemove();
    fn OnInterfaceAdd();
    fn OnInterfaceRemove();
}
pub trait IDot11AdHocNetworkImpl: Sized {
    fn GetStatus();
    fn GetSSID();
    fn HasProfile();
    fn GetProfileName();
    fn DeleteProfile();
    fn GetSignalQuality();
    fn GetSecuritySetting();
    fn GetContextGuid();
    fn GetSignature();
    fn GetInterface();
    fn Connect();
    fn Disconnect();
}
pub trait IDot11AdHocNetworkNotificationSinkImpl: Sized {
    fn OnStatusChange();
    fn OnConnectFail();
}
pub trait IDot11AdHocSecuritySettingsImpl: Sized {
    fn GetDot11AuthAlgorithm();
    fn GetDot11CipherAlgorithm();
}
pub trait IEnumDot11AdHocInterfacesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDot11AdHocNetworksImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumDot11AdHocSecuritySettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
