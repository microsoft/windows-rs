#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DialApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialApp {}
impl ::core::clone::Clone for DialApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialAppLaunchResult(pub i32);
impl DialAppLaunchResult {
    pub const Launched: Self = Self(0i32);
    pub const FailedToLaunch: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppLaunchResult {}
impl ::core::clone::Clone for DialAppLaunchResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialAppState(pub i32);
impl DialAppState {
    pub const Unknown: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Running: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppState {}
impl ::core::clone::Clone for DialAppState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialAppStateDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialAppStateDetails {}
impl ::core::clone::Clone for DialAppStateDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialAppStopResult(pub i32);
impl DialAppStopResult {
    pub const Stopped: Self = Self(0i32);
    pub const StopFailed: Self = Self(1i32);
    pub const OperationNotSupported: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for DialAppStopResult {}
impl ::core::clone::Clone for DialAppStopResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialDevice {}
impl ::core::clone::Clone for DialDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDeviceDisplayStatus(pub i32);
impl DialDeviceDisplayStatus {
    pub const None: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Disconnected: Self = Self(4i32);
    pub const Error: Self = Self(5i32);
}
impl ::core::marker::Copy for DialDeviceDisplayStatus {}
impl ::core::clone::Clone for DialDeviceDisplayStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDevicePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialDevicePicker {}
impl ::core::clone::Clone for DialDevicePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDevicePickerFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialDevicePickerFilter {}
impl ::core::clone::Clone for DialDevicePickerFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialDeviceSelectedEventArgs {}
impl ::core::clone::Clone for DialDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialDisconnectButtonClickedEventArgs {}
impl ::core::clone::Clone for DialDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DialReceiverApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DialReceiverApp {}
impl ::core::clone::Clone for DialReceiverApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialApp {}
impl ::core::clone::Clone for IDialApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialAppStateDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialAppStateDetails {}
impl ::core::clone::Clone for IDialAppStateDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDevice {}
impl ::core::clone::Clone for IDialDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDevice2 {}
impl ::core::clone::Clone for IDialDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDevicePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDevicePicker {}
impl ::core::clone::Clone for IDialDevicePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDevicePickerFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDevicePickerFilter {}
impl ::core::clone::Clone for IDialDevicePickerFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDeviceSelectedEventArgs {}
impl ::core::clone::Clone for IDialDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDeviceStatics {}
impl ::core::clone::Clone for IDialDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialDisconnectButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialDisconnectButtonClickedEventArgs {}
impl ::core::clone::Clone for IDialDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialReceiverApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialReceiverApp {}
impl ::core::clone::Clone for IDialReceiverApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialReceiverApp2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialReceiverApp2 {}
impl ::core::clone::Clone for IDialReceiverApp2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDialReceiverAppStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDialReceiverAppStatics {}
impl ::core::clone::Clone for IDialReceiverAppStatics {
    fn clone(&self) -> Self {
        *self
    }
}
