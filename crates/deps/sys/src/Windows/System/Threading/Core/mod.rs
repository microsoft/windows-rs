#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPreallocatedWorkItem(pub *mut ::core::ffi::c_void);
pub struct IPreallocatedWorkItemFactory(pub *mut ::core::ffi::c_void);
pub struct ISignalNotifier(pub *mut ::core::ffi::c_void);
pub struct ISignalNotifierStatics(pub *mut ::core::ffi::c_void);
pub struct PreallocatedWorkItem(i32);
pub struct SignalHandler(pub *mut ::core::ffi::c_void);
pub struct SignalNotifier(i32);
