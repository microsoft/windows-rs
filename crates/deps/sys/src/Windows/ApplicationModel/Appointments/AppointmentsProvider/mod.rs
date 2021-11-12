#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AddAppointmentOperation {}
impl ::core::clone::Clone for AddAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAddAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAddAppointmentOperation {}
impl ::core::clone::Clone for IAddAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderLaunchActionVerbsStatics {}
impl ::core::clone::Clone for IAppointmentsProviderLaunchActionVerbsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentsProviderLaunchActionVerbsStatics2 {}
impl ::core::clone::Clone for IAppointmentsProviderLaunchActionVerbsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoveAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoveAppointmentOperation {}
impl ::core::clone::Clone for IRemoveAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReplaceAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReplaceAppointmentOperation {}
impl ::core::clone::Clone for IReplaceAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoveAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoveAppointmentOperation {}
impl ::core::clone::Clone for RemoveAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReplaceAppointmentOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReplaceAppointmentOperation {}
impl ::core::clone::Clone for ReplaceAppointmentOperation {
    fn clone(&self) -> Self {
        *self
    }
}
