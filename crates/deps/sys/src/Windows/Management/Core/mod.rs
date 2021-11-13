#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ApplicationDataManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationDataManager {}
impl ::core::clone::Clone for ApplicationDataManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationDataManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationDataManager {}
impl ::core::clone::Clone for IApplicationDataManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationDataManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationDataManagerStatics {}
impl ::core::clone::Clone for IApplicationDataManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
