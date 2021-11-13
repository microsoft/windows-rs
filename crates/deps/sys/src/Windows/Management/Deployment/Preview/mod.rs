#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IClassicAppManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClassicAppManagerStatics {}
impl ::core::clone::Clone for IClassicAppManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInstalledClassicAppInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInstalledClassicAppInfo {}
impl ::core::clone::Clone for IInstalledClassicAppInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InstalledClassicAppInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InstalledClassicAppInfo {}
impl ::core::clone::Clone for InstalledClassicAppInfo {
    fn clone(&self) -> Self {
        *self
    }
}
