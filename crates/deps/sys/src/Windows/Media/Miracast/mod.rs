#![allow(non_snake_case, non_camel_case_types)]
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
pub struct MiracastReceiverApplySettingsStatus(i32);
pub struct MiracastReceiverAuthorizationMethod(i32);
#[repr(transparent)]
pub struct MiracastReceiverConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverConnectionCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverCursorImageChannelSettings(pub *mut ::core::ffi::c_void);
pub struct MiracastReceiverDisconnectReason(i32);
#[repr(transparent)]
pub struct MiracastReceiverDisconnectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverGameControllerDevice(pub *mut ::core::ffi::c_void);
pub struct MiracastReceiverGameControllerDeviceUsageMode(i32);
#[repr(transparent)]
pub struct MiracastReceiverInputDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverKeyboardDevice(pub *mut ::core::ffi::c_void);
pub struct MiracastReceiverListeningStatus(i32);
#[repr(transparent)]
pub struct MiracastReceiverMediaSourceCreatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverSessionStartResult(pub *mut ::core::ffi::c_void);
pub struct MiracastReceiverSessionStartStatus(i32);
#[repr(transparent)]
pub struct MiracastReceiverSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverStreamControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MiracastReceiverVideoStreamSettings(pub *mut ::core::ffi::c_void);
pub struct MiracastReceiverWiFiStatus(i32);
#[repr(transparent)]
pub struct MiracastTransmitter(pub *mut ::core::ffi::c_void);
pub struct MiracastTransmitterAuthorizationStatus(i32);
