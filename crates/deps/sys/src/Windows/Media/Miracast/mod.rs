#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMiracastReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiver {}
impl ::core::clone::Clone for IMiracastReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverApplySettingsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverApplySettingsResult {}
impl ::core::clone::Clone for IMiracastReceiverApplySettingsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverConnection {}
impl ::core::clone::Clone for IMiracastReceiverConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverConnectionCreatedEventArgs {}
impl ::core::clone::Clone for IMiracastReceiverConnectionCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverCursorImageChannel {}
impl ::core::clone::Clone for IMiracastReceiverCursorImageChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverCursorImageChannelSettings {}
impl ::core::clone::Clone for IMiracastReceiverCursorImageChannelSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverDisconnectedEventArgs {}
impl ::core::clone::Clone for IMiracastReceiverDisconnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverGameControllerDevice {}
impl ::core::clone::Clone for IMiracastReceiverGameControllerDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverInputDevices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverInputDevices {}
impl ::core::clone::Clone for IMiracastReceiverInputDevices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverKeyboardDevice {}
impl ::core::clone::Clone for IMiracastReceiverKeyboardDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverMediaSourceCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverMediaSourceCreatedEventArgs {}
impl ::core::clone::Clone for IMiracastReceiverMediaSourceCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverSession {}
impl ::core::clone::Clone for IMiracastReceiverSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverSessionStartResult {}
impl ::core::clone::Clone for IMiracastReceiverSessionStartResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverSettings {}
impl ::core::clone::Clone for IMiracastReceiverSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverStatus {}
impl ::core::clone::Clone for IMiracastReceiverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverStreamControl {}
impl ::core::clone::Clone for IMiracastReceiverStreamControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastReceiverVideoStreamSettings {}
impl ::core::clone::Clone for IMiracastReceiverVideoStreamSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMiracastTransmitter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMiracastTransmitter {}
impl ::core::clone::Clone for IMiracastTransmitter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiver {}
impl ::core::clone::Clone for MiracastReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverApplySettingsResult {}
impl ::core::clone::Clone for MiracastReceiverApplySettingsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverApplySettingsStatus(pub i32);
impl MiracastReceiverApplySettingsStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const FriendlyNameTooLong: Self = Self(4i32);
    pub const ModelNameTooLong: Self = Self(5i32);
    pub const ModelNumberTooLong: Self = Self(6i32);
    pub const InvalidSettings: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverApplySettingsStatus {}
impl ::core::clone::Clone for MiracastReceiverApplySettingsStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverAuthorizationMethod(pub i32);
impl MiracastReceiverAuthorizationMethod {
    pub const None: Self = Self(0i32);
    pub const ConfirmConnection: Self = Self(1i32);
    pub const PinDisplayIfRequested: Self = Self(2i32);
    pub const PinDisplayRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverAuthorizationMethod {}
impl ::core::clone::Clone for MiracastReceiverAuthorizationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverConnection {}
impl ::core::clone::Clone for MiracastReceiverConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverConnectionCreatedEventArgs {}
impl ::core::clone::Clone for MiracastReceiverConnectionCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverCursorImageChannel {}
impl ::core::clone::Clone for MiracastReceiverCursorImageChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverCursorImageChannelSettings {}
impl ::core::clone::Clone for MiracastReceiverCursorImageChannelSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverDisconnectReason(pub i32);
impl MiracastReceiverDisconnectReason {
    pub const Finished: Self = Self(0i32);
    pub const AppSpecificError: Self = Self(1i32);
    pub const ConnectionNotAccepted: Self = Self(2i32);
    pub const DisconnectedByUser: Self = Self(3i32);
    pub const FailedToStartStreaming: Self = Self(4i32);
    pub const MediaDecodingError: Self = Self(5i32);
    pub const MediaStreamingError: Self = Self(6i32);
    pub const MediaDecryptionError: Self = Self(7i32);
}
impl ::core::marker::Copy for MiracastReceiverDisconnectReason {}
impl ::core::clone::Clone for MiracastReceiverDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverDisconnectedEventArgs {}
impl ::core::clone::Clone for MiracastReceiverDisconnectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverGameControllerDevice {}
impl ::core::clone::Clone for MiracastReceiverGameControllerDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDeviceUsageMode(pub i32);
impl MiracastReceiverGameControllerDeviceUsageMode {
    pub const AsGameController: Self = Self(0i32);
    pub const AsMouseAndKeyboard: Self = Self(1i32);
}
impl ::core::marker::Copy for MiracastReceiverGameControllerDeviceUsageMode {}
impl ::core::clone::Clone for MiracastReceiverGameControllerDeviceUsageMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverInputDevices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverInputDevices {}
impl ::core::clone::Clone for MiracastReceiverInputDevices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverKeyboardDevice {}
impl ::core::clone::Clone for MiracastReceiverKeyboardDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverListeningStatus(pub i32);
impl MiracastReceiverListeningStatus {
    pub const NotListening: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const ConnectionPending: Self = Self(2i32);
    pub const Connected: Self = Self(3i32);
    pub const DisabledByPolicy: Self = Self(4i32);
    pub const TemporarilyDisabled: Self = Self(5i32);
}
impl ::core::marker::Copy for MiracastReceiverListeningStatus {}
impl ::core::clone::Clone for MiracastReceiverListeningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverMediaSourceCreatedEventArgs {}
impl ::core::clone::Clone for MiracastReceiverMediaSourceCreatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverSession {}
impl ::core::clone::Clone for MiracastReceiverSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverSessionStartResult {}
impl ::core::clone::Clone for MiracastReceiverSessionStartResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverSessionStartStatus(pub i32);
impl MiracastReceiverSessionStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const MiracastNotSupported: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverSessionStartStatus {}
impl ::core::clone::Clone for MiracastReceiverSessionStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverSettings {}
impl ::core::clone::Clone for MiracastReceiverSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverStatus {}
impl ::core::clone::Clone for MiracastReceiverStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverStreamControl {}
impl ::core::clone::Clone for MiracastReceiverStreamControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastReceiverVideoStreamSettings {}
impl ::core::clone::Clone for MiracastReceiverVideoStreamSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastReceiverWiFiStatus(pub i32);
impl MiracastReceiverWiFiStatus {
    pub const MiracastSupportUndetermined: Self = Self(0i32);
    pub const MiracastNotSupported: Self = Self(1i32);
    pub const MiracastSupportNotOptimized: Self = Self(2i32);
    pub const MiracastSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastReceiverWiFiStatus {}
impl ::core::clone::Clone for MiracastReceiverWiFiStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastTransmitter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MiracastTransmitter {}
impl ::core::clone::Clone for MiracastTransmitter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MiracastTransmitterAuthorizationStatus(pub i32);
impl MiracastTransmitterAuthorizationStatus {
    pub const Undecided: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const AlwaysPrompt: Self = Self(2i32);
    pub const Blocked: Self = Self(3i32);
}
impl ::core::marker::Copy for MiracastTransmitterAuthorizationStatus {}
impl ::core::clone::Clone for MiracastTransmitterAuthorizationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
