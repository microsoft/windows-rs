#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CorePerceptionAutomation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorePerceptionAutomationStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PerceptionAutomationCoreContract(i32);
