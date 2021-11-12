#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct IThreadPoolStatics(i32);
pub struct IThreadPoolTimer(i32);
pub struct IThreadPoolTimerStatics(i32);
pub struct ThreadPool(i32);
pub struct ThreadPoolTimer(i32);
pub struct TimerDestroyedHandler(i32);
pub struct TimerElapsedHandler(i32);
pub struct WorkItemHandler(i32);
pub struct WorkItemOptions(i32);
pub struct WorkItemPriority(i32);
