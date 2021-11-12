#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPreallocatedWorkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreallocatedWorkItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISignalNotifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISignalNotifierStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PreallocatedWorkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SignalHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SignalNotifier(pub *mut ::core::ffi::c_void);
