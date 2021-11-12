#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMiracastReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverApplySettingsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverInputDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMiracastTransmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsStatus(pub i32);
impl MiracastReceiverApplySettingsStatus {
    pub const Success: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(0i32);
    pub const UnknownFailure: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(1i32);
    pub const MiracastNotSupported: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(2i32);
    pub const AccessDenied: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(3i32);
    pub const FriendlyNameTooLong: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(4i32);
    pub const ModelNameTooLong: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(5i32);
    pub const ModelNumberTooLong: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(6i32);
    pub const InvalidSettings: MiracastReceiverApplySettingsStatus = MiracastReceiverApplySettingsStatus(7i32);
}
#[repr(transparent)]
pub struct MiracastReceiverAuthorizationMethod(pub i32);
impl MiracastReceiverAuthorizationMethod {
    pub const None: MiracastReceiverAuthorizationMethod = MiracastReceiverAuthorizationMethod(0i32);
    pub const ConfirmConnection: MiracastReceiverAuthorizationMethod = MiracastReceiverAuthorizationMethod(1i32);
    pub const PinDisplayIfRequested: MiracastReceiverAuthorizationMethod = MiracastReceiverAuthorizationMethod(2i32);
    pub const PinDisplayRequired: MiracastReceiverAuthorizationMethod = MiracastReceiverAuthorizationMethod(3i32);
}
#[repr(transparent)]
pub struct MiracastReceiverConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverDisconnectReason(pub i32);
impl MiracastReceiverDisconnectReason {
    pub const Finished: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(0i32);
    pub const AppSpecificError: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(1i32);
    pub const ConnectionNotAccepted: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(2i32);
    pub const DisconnectedByUser: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(3i32);
    pub const FailedToStartStreaming: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(4i32);
    pub const MediaDecodingError: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(5i32);
    pub const MediaStreamingError: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(6i32);
    pub const MediaDecryptionError: MiracastReceiverDisconnectReason = MiracastReceiverDisconnectReason(7i32);
}
#[repr(transparent)]
pub struct MiracastReceiverDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDeviceUsageMode(pub i32);
impl MiracastReceiverGameControllerDeviceUsageMode {
    pub const AsGameController: MiracastReceiverGameControllerDeviceUsageMode = MiracastReceiverGameControllerDeviceUsageMode(0i32);
    pub const AsMouseAndKeyboard: MiracastReceiverGameControllerDeviceUsageMode = MiracastReceiverGameControllerDeviceUsageMode(1i32);
}
#[repr(transparent)]
pub struct MiracastReceiverInputDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverListeningStatus(pub i32);
impl MiracastReceiverListeningStatus {
    pub const NotListening: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(0i32);
    pub const Listening: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(1i32);
    pub const ConnectionPending: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(2i32);
    pub const Connected: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(3i32);
    pub const DisabledByPolicy: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(4i32);
    pub const TemporarilyDisabled: MiracastReceiverListeningStatus = MiracastReceiverListeningStatus(5i32);
}
#[repr(transparent)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSessionStartStatus(pub i32);
impl MiracastReceiverSessionStartStatus {
    pub const Success: MiracastReceiverSessionStartStatus = MiracastReceiverSessionStartStatus(0i32);
    pub const UnknownFailure: MiracastReceiverSessionStartStatus = MiracastReceiverSessionStartStatus(1i32);
    pub const MiracastNotSupported: MiracastReceiverSessionStartStatus = MiracastReceiverSessionStartStatus(2i32);
    pub const AccessDenied: MiracastReceiverSessionStartStatus = MiracastReceiverSessionStartStatus(3i32);
}
#[repr(transparent)]
pub struct MiracastReceiverSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverWiFiStatus(pub i32);
impl MiracastReceiverWiFiStatus {
    pub const MiracastSupportUndetermined: MiracastReceiverWiFiStatus = MiracastReceiverWiFiStatus(0i32);
    pub const MiracastNotSupported: MiracastReceiverWiFiStatus = MiracastReceiverWiFiStatus(1i32);
    pub const MiracastSupportNotOptimized: MiracastReceiverWiFiStatus = MiracastReceiverWiFiStatus(2i32);
    pub const MiracastSupported: MiracastReceiverWiFiStatus = MiracastReceiverWiFiStatus(3i32);
}
#[repr(transparent)]
pub struct MiracastTransmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastTransmitterAuthorizationStatus(pub i32);
impl MiracastTransmitterAuthorizationStatus {
    pub const Undecided: MiracastTransmitterAuthorizationStatus = MiracastTransmitterAuthorizationStatus(0i32);
    pub const Allowed: MiracastTransmitterAuthorizationStatus = MiracastTransmitterAuthorizationStatus(1i32);
    pub const AlwaysPrompt: MiracastTransmitterAuthorizationStatus = MiracastTransmitterAuthorizationStatus(2i32);
    pub const Blocked: MiracastTransmitterAuthorizationStatus = MiracastTransmitterAuthorizationStatus(3i32);
}
