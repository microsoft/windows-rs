#[cfg(feature = "implement_exclusive")]
pub trait IPreallocatedWorkItemImpl: Sized {
    fn RunAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPreallocatedWorkItemFactoryImpl: Sized {
    fn CreateWorkItem(&self, handler: &::core::option::Option<super::WorkItemHandler>) -> ::windows::core::Result<PreallocatedWorkItem>;
    fn CreateWorkItemWithPriority(&self, handler: &::core::option::Option<super::WorkItemHandler>, priority: super::WorkItemPriority) -> ::windows::core::Result<PreallocatedWorkItem>;
    fn CreateWorkItemWithPriorityAndOptions(&self, handler: &::core::option::Option<super::WorkItemHandler>, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> ::windows::core::Result<PreallocatedWorkItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISignalNotifierImpl: Sized {
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Terminate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISignalNotifierStaticsImpl: Sized {
    fn AttachToEvent(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToEventWithTimeout(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>, timeout: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToSemaphore(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>) -> ::windows::core::Result<SignalNotifier>;
    fn AttachToSemaphoreWithTimeout(&self, name: &::windows::core::HSTRING, handler: &::core::option::Option<SignalHandler>, timeout: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier>;
}
