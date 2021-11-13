#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IServiceDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceDeviceStatics {}
impl ::core::clone::Clone for IServiceDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageDeviceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageDeviceStatics {}
impl ::core::clone::Clone for IStorageDeviceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServiceDeviceType(pub i32);
impl ServiceDeviceType {
    pub const CalendarService: Self = Self(0i32);
    pub const ContactsService: Self = Self(1i32);
    pub const DeviceStatusService: Self = Self(2i32);
    pub const NotesService: Self = Self(3i32);
    pub const RingtonesService: Self = Self(4i32);
    pub const SmsService: Self = Self(5i32);
    pub const TasksService: Self = Self(6i32);
}
impl ::core::marker::Copy for ServiceDeviceType {}
impl ::core::clone::Clone for ServiceDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
