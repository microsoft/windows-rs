#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ListId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompletedDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetCompletedDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDetails(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DetailsKind(&self) -> ::windows::core::Result<UserDataTaskDetailsKind>;
    fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> ::windows::core::Result<()>;
    fn DueDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDueDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<UserDataTaskKind>;
    fn Priority(&self) -> ::windows::core::Result<UserDataTaskPriority>;
    fn SetPriority(&self, value: UserDataTaskPriority) -> ::windows::core::Result<()>;
    fn RecurrenceProperties(&self) -> ::windows::core::Result<UserDataTaskRecurrenceProperties>;
    fn SetRecurrenceProperties(&self, value: &::core::option::Option<UserDataTaskRecurrenceProperties>) -> ::windows::core::Result<()>;
    fn RegenerationProperties(&self) -> ::windows::core::Result<UserDataTaskRegenerationProperties>;
    fn SetRegenerationProperties(&self, value: &::core::option::Option<UserDataTaskRegenerationProperties>) -> ::windows::core::Result<()>;
    fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReminder(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Sensitivity(&self) -> ::windows::core::Result<UserDataTaskSensitivity>;
    fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> ::windows::core::Result<()>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetStartDate(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskBatchImpl: Sized {
    fn Tasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherAppReadAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn LimitedWriteOperations(&self) -> ::windows::core::Result<UserDataTaskListLimitedWriteOperations>;
    fn SyncManager(&self) -> ::windows::core::Result<UserDataTaskListSyncManager>;
    fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetTaskReader(&self) -> ::windows::core::Result<UserDataTaskReader>;
    fn GetTaskReaderWithOptions(&self, options: &::core::option::Option<UserDataTaskQueryOptions>) -> ::windows::core::Result<UserDataTaskReader>;
    fn GetTaskAsync(&self, userdatatask: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTask>>;
    fn SaveTaskAsync(&self, userdatatask: &::core::option::Option<UserDataTask>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteTaskAsync(&self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListLimitedWriteOperationsImpl: Sized {
    fn TryCompleteTaskAsync(&self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn TryCreateOrUpdateTaskAsync(&self, userdatatask: &::core::option::Option<UserDataTask>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDeleteTaskAsync(&self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySkipOccurrenceAsync(&self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskListSyncManagerImpl: Sized {
    fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastAttemptedSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastSuccessfulSyncTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<UserDataTaskListSyncStatus>;
    fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> ::windows::core::Result<()>;
    fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskManagerImpl: Sized {
    fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>>;
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<UserDataTaskManager>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserDataTaskManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskQueryOptionsImpl: Sized {
    fn SortProperty(&self) -> ::windows::core::Result<UserDataTaskQuerySortProperty>;
    fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> ::windows::core::Result<()>;
    fn Kind(&self) -> ::windows::core::Result<UserDataTaskQueryKind>;
    fn SetKind(&self, value: UserDataTaskQueryKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskReaderImpl: Sized {
    fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskRecurrencePropertiesImpl: Sized {
    fn Unit(&self) -> ::windows::core::Result<UserDataTaskRecurrenceUnit>;
    fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetOccurrences(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<i32>;
    fn SetInterval(&self, value: i32) -> ::windows::core::Result<()>;
    fn DaysOfWeek(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>;
    fn SetDaysOfWeek(&self, value: &::core::option::Option<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>) -> ::windows::core::Result<()>;
    fn WeekOfMonth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>;
    fn SetWeekOfMonth(&self, value: &::core::option::Option<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>) -> ::windows::core::Result<()>;
    fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMonth(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetDay(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskRegenerationPropertiesImpl: Sized {
    fn Unit(&self) -> ::windows::core::Result<UserDataTaskRegenerationUnit>;
    fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetOccurrences(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<i32>;
    fn SetInterval(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskStoreImpl: Sized {
    fn CreateListAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
    fn CreateListInAccountAsync(&self, name: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
    fn FindListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>>;
    fn GetListAsync(&self, tasklistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
}
