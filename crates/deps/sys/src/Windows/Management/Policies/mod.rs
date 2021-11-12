#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct INamedPolicyData(pub *mut ::core::ffi::c_void);
pub struct INamedPolicyStatics(pub *mut ::core::ffi::c_void);
pub struct NamedPolicy(i32);
pub struct NamedPolicyData(i32);
pub struct NamedPolicyKind(i32);
