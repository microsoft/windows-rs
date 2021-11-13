#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInstalledDesktopApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInstalledDesktopApp {}
impl ::core::clone::Clone for IInstalledDesktopApp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInstalledDesktopAppStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInstalledDesktopAppStatics {}
impl ::core::clone::Clone for IInstalledDesktopAppStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InstalledDesktopApp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InstalledDesktopApp {}
impl ::core::clone::Clone for InstalledDesktopApp {
    fn clone(&self) -> Self {
        *self
    }
}
