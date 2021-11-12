#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IXamlDirect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlDirectObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlDirectStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XamlDirect(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XamlDirectContract(i32);
#[repr(C)]
pub struct XamlEventIndex(i32);
#[repr(C)]
pub struct XamlPropertyIndex(i32);
#[repr(C)]
pub struct XamlTypeIndex(i32);
