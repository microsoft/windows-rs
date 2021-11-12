#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskBatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskDaysOfWeek(pub u32);
impl UserDataTaskDaysOfWeek {
    pub const None: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(0u32);
    pub const Sunday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(1u32);
    pub const Monday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(2u32);
    pub const Tuesday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(4u32);
    pub const Wednesday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(8u32);
    pub const Thursday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(16u32);
    pub const Friday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(32u32);
    pub const Saturday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(64u32);
}
#[repr(transparent)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: UserDataTaskDetailsKind = UserDataTaskDetailsKind(0i32);
    pub const Html: UserDataTaskDetailsKind = UserDataTaskDetailsKind(1i32);
}
#[repr(transparent)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: UserDataTaskKind = UserDataTaskKind(0i32);
    pub const Recurring: UserDataTaskKind = UserDataTaskKind(1i32);
    pub const Regenerating: UserDataTaskKind = UserDataTaskKind(2i32);
}
#[repr(transparent)]
pub struct UserDataTaskList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(0i32);
    pub const SystemOnly: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(1i32);
    pub const None: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(2i32);
}
#[repr(transparent)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(0i32);
    pub const None: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(1i32);
}
#[repr(transparent)]
pub struct UserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListSyncStatus(pub i32);
impl UserDataTaskListSyncStatus {
    pub const Idle: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(0i32);
    pub const Syncing: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(1i32);
    pub const UpToDate: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(2i32);
    pub const AuthenticationError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(3i32);
    pub const PolicyError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(4i32);
    pub const UnknownError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(5i32);
}
#[repr(transparent)]
pub struct UserDataTaskManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: UserDataTaskPriority = UserDataTaskPriority(0i32);
    pub const Low: UserDataTaskPriority = UserDataTaskPriority(-1i32);
    pub const High: UserDataTaskPriority = UserDataTaskPriority(1i32);
}
#[repr(transparent)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: UserDataTaskQueryKind = UserDataTaskQueryKind(0i32);
    pub const Incomplete: UserDataTaskQueryKind = UserDataTaskQueryKind(1i32);
    pub const Complete: UserDataTaskQueryKind = UserDataTaskQueryKind(2i32);
}
#[repr(transparent)]
pub struct UserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: UserDataTaskQuerySortProperty = UserDataTaskQuerySortProperty(0i32);
}
#[repr(transparent)]
pub struct UserDataTaskReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskRecurrenceUnit(pub i32);
impl UserDataTaskRecurrenceUnit {
    pub const Daily: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(0i32);
    pub const Weekly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(1i32);
    pub const Monthly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(2i32);
    pub const MonthlyOnDay: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(3i32);
    pub const Yearly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(4i32);
    pub const YearlyOnDay: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(5i32);
}
#[repr(transparent)]
pub struct UserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(0i32);
    pub const Weekly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(1i32);
    pub const Monthly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(2i32);
    pub const Yearly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(4i32);
}
#[repr(transparent)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: UserDataTaskSensitivity = UserDataTaskSensitivity(0i32);
    pub const Private: UserDataTaskSensitivity = UserDataTaskSensitivity(1i32);
}
#[repr(transparent)]
pub struct UserDataTaskStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(0i32);
    pub const AllTasksLimitedReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(1i32);
}
#[repr(transparent)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(0i32);
    pub const Second: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(1i32);
    pub const Third: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(2i32);
    pub const Fourth: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(3i32);
    pub const Last: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(4i32);
}
