#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolStaticsImpl: Sized {
    fn RunAsync(&self, handler: &::core::option::Option<WorkItemHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAsync(&self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAndOptionsAsync(&self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority, options: WorkItemOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolTimerImpl: Sized {
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolTimerStaticsImpl: Sized {
    fn CreatePeriodicTimer(&self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimer(&self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreatePeriodicTimerWithCompletion(&self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimerWithCompletion(&self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
}
