#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn IThreadPoolStatics();
    fn IThreadPoolTimer();
    fn IThreadPoolTimerStatics();
    fn ThreadPool();
    fn ThreadPoolTimer();
    fn TimerDestroyedHandler();
    fn TimerElapsedHandler();
    fn WorkItemHandler();
    fn WorkItemOptions();
    fn WorkItemPriority();
}
