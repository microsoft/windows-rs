#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CastingConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingConnection {}
impl ::core::clone::Clone for CastingConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingConnectionErrorOccurredEventArgs {}
impl ::core::clone::Clone for CastingConnectionErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const DeviceDidNotRespond: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
    pub const InvalidCastingSource: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for CastingConnectionErrorStatus {}
impl ::core::clone::Clone for CastingConnectionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Connecting: Self = Self(4i32);
}
impl ::core::marker::Copy for CastingConnectionState {}
impl ::core::clone::Clone for CastingConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingDevice {}
impl ::core::clone::Clone for CastingDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingDevicePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingDevicePicker {}
impl ::core::clone::Clone for CastingDevicePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingDevicePickerFilter {}
impl ::core::clone::Clone for CastingDevicePickerFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingDeviceSelectedEventArgs {}
impl ::core::clone::Clone for CastingDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const Picture: Self = Self(4u32);
}
impl ::core::marker::Copy for CastingPlaybackTypes {}
impl ::core::clone::Clone for CastingPlaybackTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CastingSource {}
impl ::core::clone::Clone for CastingSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingConnection {}
impl ::core::clone::Clone for ICastingConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingConnectionErrorOccurredEventArgs {}
impl ::core::clone::Clone for ICastingConnectionErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingDevice {}
impl ::core::clone::Clone for ICastingDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingDevicePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingDevicePicker {}
impl ::core::clone::Clone for ICastingDevicePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingDevicePickerFilter {}
impl ::core::clone::Clone for ICastingDevicePickerFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingDeviceSelectedEventArgs {}
impl ::core::clone::Clone for ICastingDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingDeviceStatics {}
impl ::core::clone::Clone for ICastingDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICastingSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICastingSource {}
impl ::core::clone::Clone for ICastingSource {
    fn clone(&self) -> Self {
        *self
    }
}
