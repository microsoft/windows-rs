#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationImpl: Sized {
    fn ServiceAuthenticationHmac(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn SessionNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn FinishAuthenticationAsync(&self, devicehmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, sessionhmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>;
    fn AbortAuthenticationAsync(&self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStatus>;
    fn Authentication(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthentication>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsImpl: Sized {
    fn StageInfo(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStageInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStageInfoImpl: Sized {
    fn Stage(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStage>;
    fn Scenario(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationScenario>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStaticsImpl: Sized {
    fn ShowNotificationMessageAsync(&self, devicename: &::windows::core::HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn StartAuthenticationAsync(&self, deviceid: &::windows::core::HSTRING, serviceauthenticationnonce: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>;
    fn AuthenticationStageChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStageChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAuthenticationStageInfoAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl: Sized {
    fn RegisterDevicePresenceMonitoringAsync(&self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn RegisterDevicePresenceMonitoringWithNewDeviceAsync(&self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn UnregisterDevicePresenceMonitoringAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsDevicePresenceMonitoringSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorInfoImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorInfo2Impl: Sized + ISecondaryAuthenticationFactorInfoImpl {
    fn PresenceMonitoringMode(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>;
    fn UpdateDevicePresenceAsync(&self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsAuthenticationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistrationImpl: Sized {
    fn FinishRegisteringDeviceAsync(&self, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AbortRegisteringDeviceAsync(&self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistrationStatus>;
    fn Registration(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistration>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistrationStaticsImpl: Sized {
    fn RequestStartRegisteringDeviceAsync(&self, deviceid: &::windows::core::HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, devicekey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, mutualauthenticationkey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>;
    fn FindAllRegisteredDeviceInfoAsync(&self, querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>;
    fn UnregisterDeviceAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn UpdateDeviceConfigurationDataAsync(&self, deviceid: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
