#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMdmAllowPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMdmPolicyStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkplaceSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessagingSyncPolicy(pub i32);
impl MessagingSyncPolicy {
    pub const Disallowed: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MessagingSyncPolicy {}
impl ::core::clone::Clone for MessagingSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
