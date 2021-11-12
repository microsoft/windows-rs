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
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MiracastReceiverSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MiracastReceiverStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
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
