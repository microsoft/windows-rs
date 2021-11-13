#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for AddContactResult {}
impl ::core::clone::Clone for AddContactResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactPickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactPickerUI {}
impl ::core::clone::Clone for ContactPickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactRemovedEventArgs {}
impl ::core::clone::Clone for ContactRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPickerUI(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPickerUI {}
impl ::core::clone::Clone for IContactPickerUI {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactPickerUI2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactPickerUI2 {}
impl ::core::clone::Clone for IContactPickerUI2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactRemovedEventArgs {}
impl ::core::clone::Clone for IContactRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
