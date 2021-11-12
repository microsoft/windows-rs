#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IServiceDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PortableDeviceContract(i32);
#[repr(transparent)]
pub struct ServiceDeviceType(pub i32);
impl ServiceDeviceType {
    pub const CalendarService: ServiceDeviceType = ServiceDeviceType(0i32);
    pub const ContactsService: ServiceDeviceType = ServiceDeviceType(1i32);
    pub const DeviceStatusService: ServiceDeviceType = ServiceDeviceType(2i32);
    pub const NotesService: ServiceDeviceType = ServiceDeviceType(3i32);
    pub const RingtonesService: ServiceDeviceType = ServiceDeviceType(4i32);
    pub const SmsService: ServiceDeviceType = ServiceDeviceType(5i32);
    pub const TasksService: ServiceDeviceType = ServiceDeviceType(6i32);
}
