#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IThreadPoolStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThreadPoolTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThreadPoolTimerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ThreadPool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ThreadPoolTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimerDestroyedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimerElapsedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WorkItemHandler(pub *mut ::core::ffi::c_void);
pub struct WorkItemOptions(i32);
pub struct WorkItemPriority(i32);
