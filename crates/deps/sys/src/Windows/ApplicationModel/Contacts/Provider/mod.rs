#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ContactPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactPickerUI2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
