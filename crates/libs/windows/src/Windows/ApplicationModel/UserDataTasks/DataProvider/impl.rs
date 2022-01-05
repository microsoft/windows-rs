#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskDataProviderConnectionImpl: Sized {
    fn CreateOrUpdateTaskRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCreateOrUpdateTaskRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateOrUpdateTaskRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SkipOccurrenceRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSkipOccurrenceRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSkipOccurrenceRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CompleteTaskRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCompleteTaskRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleteTaskRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteTaskRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListDeleteTaskRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteTaskRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<UserDataTaskDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListCompleteTaskRequestImpl: Sized {
    fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self, completedtaskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListCompleteTaskRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserDataTaskListCompleteTaskRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListCreateOrUpdateTaskRequestImpl: Sized {
    fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Task(&self) -> ::windows::core::Result<super::UserDataTask>;
    fn ReportCompletedAsync(&self, createdorupdateduserdatatask: &::core::option::Option<super::UserDataTask>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListCreateOrUpdateTaskRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserDataTaskListCreateOrUpdateTaskRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListDeleteTaskRequestImpl: Sized {
    fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListDeleteTaskRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserDataTaskListDeleteTaskRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListSkipOccurrenceRequestImpl: Sized {
    fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListSkipOccurrenceRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserDataTaskListSkipOccurrenceRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListSyncManagerSyncRequestImpl: Sized {
    fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<UserDataTaskListSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
