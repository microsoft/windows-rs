#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DataProtectionProvider(i32);
pub struct IDataProtectionProvider(pub *mut ::core::ffi::c_void);
pub struct IDataProtectionProviderFactory(pub *mut ::core::ffi::c_void);
