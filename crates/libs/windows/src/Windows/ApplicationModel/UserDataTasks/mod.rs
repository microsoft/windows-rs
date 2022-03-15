#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTask {
    type Vtable = IUserDataTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6585d1_e0d4_4f99_aee2_bc2d5ddadf4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTask_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CompletedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompletedDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetCompletedDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCompletedDate: usize,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskDetailsKind) -> ::windows::core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskDetailsKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DueDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DueDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDueDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDueDate: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskKind) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskPriority) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskPriority) -> ::windows::core::HRESULT,
    pub RecurrenceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRecurrenceProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegenerationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRegenerationProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reminder: usize,
    #[cfg(feature = "Foundation")]
    pub SetReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReminder: usize,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskSensitivity) -> ::windows::core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskSensitivity) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskBatch(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x382da5fe_20b5_431c_8f42_a5d292ec930c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskBatch_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskList(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskList {
    type Vtable = IUserDataTaskList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49412e39_7c1d_4df1_bed3_314b7cbf5e4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskList_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub LimitedWriteOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
    pub GetTaskReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTaskReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListLimitedWriteOperations(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7aa267f2_6078_4183_919e_4f29f19cfae9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TryCompleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCompleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteTaskAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteTaskAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySkipOccurrenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySkipOccurrenceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e591a95_1dcf_469f_93ec_ba48bb553c6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListSyncStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskListSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskManager {
    type Vtable = IUserDataTaskManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8451c914_e60b_48a9_9211_7fb8a56cb84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: UserDataTaskStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskManagerStatics {
    type Vtable = IUserDataTaskManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35539f8_c502_47fc_a81e_100883719d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x959f27ed_909a_4d30_8c1b_331d8fe667e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQueryKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskQueryKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskReader {
    type Vtable = IUserDataTaskReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e688b1_4ccf_4500_883b_e76290cfed63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskRecurrenceProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73df80b0_27c6_40ce_b149_9cd41485a69e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DaysOfWeek: usize,
    #[cfg(feature = "Foundation")]
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDaysOfWeek: usize,
    #[cfg(feature = "Foundation")]
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WeekOfMonth: usize,
    #[cfg(feature = "Foundation")]
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWeekOfMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Month: usize,
    #[cfg(feature = "Foundation")]
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMonth: usize,
    #[cfg(feature = "Foundation")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDay: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskRegenerationProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ab0007_090e_4704_bb5c_84fc0b0d9c31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataTaskStore {
    type Vtable = IUserDataTaskStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf06a9cb0_f1db_45ba_8a62_086004c0213d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateListAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateListInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateListInAccountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindListsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tasklistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetListAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTask(::windows::core::IUnknown);
impl UserDataTask {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTask, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn ListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ListId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompletedDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CompletedDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompletedDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompletedDate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetDetails<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDetails)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn DetailsKind(&self) -> ::windows::core::Result<UserDataTaskDetailsKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskDetailsKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetailsKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskDetailsKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDetailsKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DueDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DueDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDueDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDueDate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataTaskKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Priority(&self) -> ::windows::core::Result<UserDataTaskPriority> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Priority)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskPriority>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetPriority(&self, value: UserDataTaskPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPriority)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn RecurrenceProperties(&self) -> ::windows::core::Result<UserDataTaskRecurrenceProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecurrenceProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetRecurrenceProperties<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskRecurrenceProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecurrenceProperties)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn RegenerationProperties(&self) -> ::windows::core::Result<UserDataTaskRegenerationProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegenerationProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetRegenerationProperties<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskRegenerationProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRegenerationProperties)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reminder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReminder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReminder)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Sensitivity(&self) -> ::windows::core::Result<UserDataTaskSensitivity> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskSensitivity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sensitivity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskSensitivity>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSensitivity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubject)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartDate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTask {}
impl ::core::fmt::Debug for UserDataTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTask;{7c6585d1-e0d4-4f99-aee2-bc2d5ddadf4c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTask {
    type Vtable = IUserDataTask_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTask as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTask";
}
impl ::core::convert::From<UserDataTask> for ::windows::core::IUnknown {
    fn from(value: UserDataTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows::core::IUnknown {
    fn from(value: &UserDataTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTask> for ::windows::core::IInspectable {
    fn from(value: UserDataTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows::core::IInspectable {
    fn from(value: &UserDataTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTask {}
unsafe impl ::core::marker::Sync for UserDataTask {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskBatch(::windows::core::IUnknown);
impl UserDataTaskBatch {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tasks)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UserDataTask>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskBatch {}
impl ::core::fmt::Debug for UserDataTaskBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskBatch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch;{382da5fe-20b5-431c-8f42-a5d292ec930c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskBatch as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskBatch {}
unsafe impl ::core::marker::Sync for UserDataTaskBatch {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskDaysOfWeek(pub u32);
impl UserDataTaskDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for UserDataTaskDaysOfWeek {}
impl ::core::clone::Clone for UserDataTaskDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskDaysOfWeek {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDaysOfWeek").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataTaskDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataTaskDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDaysOfWeek {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskDetailsKind {}
impl ::core::clone::Clone for UserDataTaskDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskDetailsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDetailsKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDetailsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: Self = Self(0i32);
    pub const Recurring: Self = Self(1i32);
    pub const Regenerating: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskKind {}
impl ::core::clone::Clone for UserDataTaskKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskList(::windows::core::IUnknown);
impl UserDataTaskList {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDataAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppReadAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppReadAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppWriteAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppWriteAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppWriteAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppWriteAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn LimitedWriteOperations(&self) -> ::windows::core::Result<UserDataTaskListLimitedWriteOperations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LimitedWriteOperations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListLimitedWriteOperations>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SyncManager(&self) -> ::windows::core::Result<UserDataTaskListSyncManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncManager)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncManager>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterSyncManagerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn GetTaskReader(&self) -> ::windows::core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTaskReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn GetTaskReaderWithOptions<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskQueryOptions>>(&self, options: Param0) -> ::windows::core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTaskReaderWithOptions)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTaskAsync)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTask>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveTaskAsync)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteTaskAsync)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskList {}
impl ::core::fmt::Debug for UserDataTaskList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskList;{49412e39-7c1d-4df1-bed3-314b7cbf5e4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskList {
    type Vtable = IUserDataTaskList_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
}
impl ::core::convert::From<UserDataTaskList> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskList> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskList {}
unsafe impl ::core::marker::Sync for UserDataTaskList {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListLimitedWriteOperations(::windows::core::IUnknown);
impl UserDataTaskListLimitedWriteOperations {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCompleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCompleteTaskAsync)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateOrUpdateTaskAsync)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDeleteTaskAsync)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySkipOccurrenceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySkipOccurrenceAsync)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListLimitedWriteOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListLimitedWriteOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListLimitedWriteOperations {}
impl ::core::fmt::Debug for UserDataTaskListLimitedWriteOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListLimitedWriteOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListLimitedWriteOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations;{7aa267f2-6078-4183-919e-4f29f19cfae9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskListLimitedWriteOperations as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListLimitedWriteOperations {}
unsafe impl ::core::marker::Sync for UserDataTaskListLimitedWriteOperations {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskListOtherAppReadAccess {}
impl ::core::clone::Clone for UserDataTaskListOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskListOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskListOtherAppWriteAccess {}
impl ::core::clone::Clone for UserDataTaskListOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskListOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListOtherAppWriteAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListOtherAppWriteAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListOtherAppWriteAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSyncManager(::windows::core::IUnknown);
impl UserDataTaskListSyncManager {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastAttemptedSyncTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLastAttemptedSyncTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastSuccessfulSyncTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<UserDataTaskListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListSyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSyncStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManager {}
impl ::core::fmt::Debug for UserDataTaskListSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager;{8e591a95-1dcf-469f-93ec-ba48bb553c6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskListSyncManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManager {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskListSyncStatus(pub i32);
impl UserDataTaskListSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
}
impl ::core::marker::Copy for UserDataTaskListSyncStatus {}
impl ::core::clone::Clone for UserDataTaskListSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskListSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListSyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskListSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskManager(::windows::core::IUnknown);
impl UserDataTaskManager {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskStore>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataTaskManagerStatics<R, F: FnOnce(&IUserDataTaskManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskManager, IUserDataTaskManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UserDataTaskManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskManager {}
impl ::core::fmt::Debug for UserDataTaskManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskManager;{8451c914-e60b-48a9-9211-7fb8a56cb84c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskManager {
    type Vtable = IUserDataTaskManager_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
}
impl ::core::convert::From<UserDataTaskManager> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskManager> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskManager {}
unsafe impl ::core::marker::Sync for UserDataTaskManager {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: Self = Self(0i32);
    pub const Low: Self = Self(-1i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskPriority {}
impl ::core::clone::Clone for UserDataTaskPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: Self = Self(0i32);
    pub const Incomplete: Self = Self(1i32);
    pub const Complete: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataTaskQueryKind {}
impl ::core::clone::Clone for UserDataTaskQueryKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskQueryKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskQueryKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskQueryKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQueryKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQueryKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskQueryOptions(::windows::core::IUnknown);
impl UserDataTaskQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskQueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SortProperty(&self) -> ::windows::core::Result<UserDataTaskQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQuerySortProperty = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SortProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQuerySortProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSortProperty)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataTaskQueryKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQueryKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQueryKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetKind(&self, value: UserDataTaskQueryKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKind)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskQueryOptions {}
impl ::core::fmt::Debug for UserDataTaskQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions;{959f27ed-909a-4d30-8c1b-331d8fe667e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskQueryOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskQueryOptions {}
unsafe impl ::core::marker::Sync for UserDataTaskQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: Self = Self(0i32);
}
impl ::core::marker::Copy for UserDataTaskQuerySortProperty {}
impl ::core::clone::Clone for UserDataTaskQuerySortProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskQuerySortProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskQuerySortProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskQuerySortProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskQuerySortProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQuerySortProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskReader(::windows::core::IUnknown);
impl UserDataTaskReader {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskReader {}
impl ::core::fmt::Debug for UserDataTaskReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskReader;{03e688b1-4ccf-4500-883b-e76290cfed63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskReader {
    type Vtable = IUserDataTaskReader_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
}
impl ::core::convert::From<UserDataTaskReader> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskReader> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskReader {}
unsafe impl ::core::marker::Sync for UserDataTaskReader {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskRecurrenceProperties(::windows::core::IUnknown);
impl UserDataTaskRecurrenceProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskRecurrenceProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Unit(&self) -> ::windows::core::Result<UserDataTaskRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRecurrenceUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceUnit>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occurrences)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOccurrences)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Until)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUntil)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Interval(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Interval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetInterval(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInterval)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DaysOfWeek(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DaysOfWeek)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDaysOfWeek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDaysOfWeek)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WeekOfMonth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WeekOfMonth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWeekOfMonth<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWeekOfMonth)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Month)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMonth<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMonth)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Day)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDay)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskRecurrenceProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskRecurrenceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskRecurrenceProperties {}
impl ::core::fmt::Debug for UserDataTaskRecurrenceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRecurrenceProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRecurrenceProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties;{73df80b0-27c6-40ce-b149-9cd41485a69e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskRecurrenceProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRecurrenceProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRecurrenceProperties {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskRecurrenceUnit(pub i32);
impl UserDataTaskRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for UserDataTaskRecurrenceUnit {}
impl ::core::clone::Clone for UserDataTaskRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskRecurrenceUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRecurrenceUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRecurrenceUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskRegenerationProperties(::windows::core::IUnknown);
impl UserDataTaskRegenerationProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskRegenerationProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Unit(&self) -> ::windows::core::Result<UserDataTaskRegenerationUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRegenerationUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationUnit>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Occurrences)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOccurrences)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Until)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUntil)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn Interval(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Interval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
    pub fn SetInterval(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInterval)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskRegenerationProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskRegenerationProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskRegenerationProperties {}
impl ::core::fmt::Debug for UserDataTaskRegenerationProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRegenerationProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRegenerationProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties;{92ab0007-090e-4704-bb5c-84fc0b0d9c31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskRegenerationProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRegenerationProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRegenerationProperties {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const Yearly: Self = Self(4i32);
}
impl ::core::marker::Copy for UserDataTaskRegenerationUnit {}
impl ::core::clone::Clone for UserDataTaskRegenerationUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskRegenerationUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskRegenerationUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskRegenerationUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskRegenerationUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRegenerationUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskSensitivity {}
impl ::core::clone::Clone for UserDataTaskSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskSensitivity {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskSensitivity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskSensitivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
pub struct UserDataTaskStore(::windows::core::IUnknown);
impl UserDataTaskStore {
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateListAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateListInAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateListInAccountAsync)(::core::mem::transmute_copy(this), name.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindListsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tasklistid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetListAsync)(::core::mem::transmute_copy(this), tasklistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskStore {}
impl ::core::fmt::Debug for UserDataTaskStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskStore;{f06a9cb0-f1db-45ba-8a62-086004c0213d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataTaskStore {
    type Vtable = IUserDataTaskStore_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataTaskStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
}
impl ::core::convert::From<UserDataTaskStore> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataTaskStore> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataTaskStore {}
unsafe impl ::core::marker::Sync for UserDataTaskStore {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: Self = Self(0i32);
    pub const AllTasksLimitedReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataTaskStoreAccessType {}
impl ::core::clone::Clone for UserDataTaskStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for UserDataTaskWeekOfMonth {}
impl ::core::clone::Clone for UserDataTaskWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataTaskWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskWeekOfMonth {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataTaskWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskWeekOfMonth").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskWeekOfMonth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
