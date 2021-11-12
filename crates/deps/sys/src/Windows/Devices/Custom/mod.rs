#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomDevice(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CustomDeviceContract(i32);
#[repr(transparent)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: DeviceAccessMode = DeviceAccessMode(0i32);
    pub const Write: DeviceAccessMode = DeviceAccessMode(1i32);
    pub const ReadWrite: DeviceAccessMode = DeviceAccessMode(2i32);
}
#[repr(transparent)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: DeviceSharingMode = DeviceSharingMode(0i32);
    pub const Exclusive: DeviceSharingMode = DeviceSharingMode(1i32);
}
#[repr(transparent)]
pub struct ICustomDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIOControlCode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIOControlCodeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownDeviceTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: IOControlAccessMode = IOControlAccessMode(0i32);
    pub const Read: IOControlAccessMode = IOControlAccessMode(1i32);
    pub const Write: IOControlAccessMode = IOControlAccessMode(2i32);
    pub const ReadWrite: IOControlAccessMode = IOControlAccessMode(3i32);
}
#[repr(transparent)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: IOControlBufferingMethod = IOControlBufferingMethod(0i32);
    pub const DirectInput: IOControlBufferingMethod = IOControlBufferingMethod(1i32);
    pub const DirectOutput: IOControlBufferingMethod = IOControlBufferingMethod(2i32);
    pub const Neither: IOControlBufferingMethod = IOControlBufferingMethod(3i32);
}
#[repr(transparent)]
pub struct IOControlCode(pub *mut ::core::ffi::c_void);
