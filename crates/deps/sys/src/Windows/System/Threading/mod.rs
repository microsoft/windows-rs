#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct IThreadPoolStatics(pub *mut ::core::ffi::c_void);
pub struct IThreadPoolTimer(pub *mut ::core::ffi::c_void);
pub struct IThreadPoolTimerStatics(pub *mut ::core::ffi::c_void);
pub struct ThreadPool(i32);
pub struct ThreadPoolTimer(i32);
pub struct TimerDestroyedHandler(pub *mut ::core::ffi::c_void);
pub struct TimerElapsedHandler(pub *mut ::core::ffi::c_void);
pub struct WorkItemHandler(pub *mut ::core::ffi::c_void);
pub struct WorkItemOptions(i32);
pub struct WorkItemPriority(i32);
