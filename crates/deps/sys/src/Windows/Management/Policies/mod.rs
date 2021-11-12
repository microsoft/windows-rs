#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INamedPolicyData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamedPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NamedPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NamedPolicyData(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NamedPolicyKind(i32);
