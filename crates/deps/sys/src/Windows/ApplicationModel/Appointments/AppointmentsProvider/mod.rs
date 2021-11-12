#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddAppointmentOperation(i32);
pub struct AppointmentsProviderLaunchActionVerbs(i32);
pub struct IAddAppointmentOperation(pub *mut ::core::ffi::c_void);
pub struct IAppointmentsProviderLaunchActionVerbsStatics(pub *mut ::core::ffi::c_void);
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(pub *mut ::core::ffi::c_void);
pub struct IRemoveAppointmentOperation(pub *mut ::core::ffi::c_void);
pub struct IReplaceAppointmentOperation(pub *mut ::core::ffi::c_void);
pub struct RemoveAppointmentOperation(i32);
pub struct ReplaceAppointmentOperation(i32);
