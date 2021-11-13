#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCapability {}
impl ::core::clone::Clone for AppCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppCapabilityAccessChangedEventArgs {}
impl ::core::clone::Clone for AppCapabilityAccessChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
}
impl ::core::marker::Copy for AppCapabilityAccessStatus {}
impl ::core::clone::Clone for AppCapabilityAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCapability {}
impl ::core::clone::Clone for IAppCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCapabilityAccessChangedEventArgs {}
impl ::core::clone::Clone for IAppCapabilityAccessChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppCapabilityStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppCapabilityStatics {}
impl ::core::clone::Clone for IAppCapabilityStatics {
    fn clone(&self) -> Self {
        *self
    }
}
