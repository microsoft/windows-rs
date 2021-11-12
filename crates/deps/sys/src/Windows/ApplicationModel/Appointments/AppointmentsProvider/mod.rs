#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddAppointmentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAddAppointmentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoveAppointmentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReplaceAppointmentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RemoveAppointmentOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReplaceAppointmentOperation(pub *mut ::core::ffi::c_void);
