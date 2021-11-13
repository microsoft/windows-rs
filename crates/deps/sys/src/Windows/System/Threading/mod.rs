#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IThreadPoolStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThreadPoolStatics {}
impl ::core::clone::Clone for IThreadPoolStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThreadPoolTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThreadPoolTimer {}
impl ::core::clone::Clone for IThreadPoolTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThreadPoolTimerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThreadPoolTimerStatics {}
impl ::core::clone::Clone for IThreadPoolTimerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ThreadPoolTimer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ThreadPoolTimer {}
impl ::core::clone::Clone for ThreadPoolTimer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimerDestroyedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimerDestroyedHandler {}
impl ::core::clone::Clone for TimerDestroyedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimerElapsedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimerElapsedHandler {}
impl ::core::clone::Clone for TimerElapsedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkItemHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WorkItemHandler {}
impl ::core::clone::Clone for WorkItemHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: Self = Self(0u32);
    pub const TimeSliced: Self = Self(1u32);
}
impl ::core::marker::Copy for WorkItemOptions {}
impl ::core::clone::Clone for WorkItemOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for WorkItemPriority {}
impl ::core::clone::Clone for WorkItemPriority {
    fn clone(&self) -> Self {
        *self
    }
}
