#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
pub struct ThreadPoolTimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimerDestroyedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimerElapsedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WorkItemHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: Self = Self(0u32);
    pub const TimeSliced: Self = Self(1u32);
}
#[repr(transparent)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
