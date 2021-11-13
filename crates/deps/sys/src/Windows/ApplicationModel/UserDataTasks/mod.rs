#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTask {}
impl ::core::clone::Clone for IUserDataTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskBatch {}
impl ::core::clone::Clone for IUserDataTaskBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskList {}
impl ::core::clone::Clone for IUserDataTaskList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListLimitedWriteOperations {}
impl ::core::clone::Clone for IUserDataTaskListLimitedWriteOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListSyncManager {}
impl ::core::clone::Clone for IUserDataTaskListSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskManager {}
impl ::core::clone::Clone for IUserDataTaskManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskManagerStatics {}
impl ::core::clone::Clone for IUserDataTaskManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskQueryOptions {}
impl ::core::clone::Clone for IUserDataTaskQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskReader {}
impl ::core::clone::Clone for IUserDataTaskReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskRecurrenceProperties {}
impl ::core::clone::Clone for IUserDataTaskRecurrenceProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskRegenerationProperties {}
impl ::core::clone::Clone for IUserDataTaskRegenerationProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskStore {}
impl ::core::clone::Clone for IUserDataTaskStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTask {}
impl ::core::clone::Clone for UserDataTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskBatch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskBatch {}
impl ::core::clone::Clone for UserDataTaskBatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskList {}
impl ::core::clone::Clone for UserDataTaskList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListLimitedWriteOperations {}
impl ::core::clone::Clone for UserDataTaskListLimitedWriteOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListSyncManager {}
impl ::core::clone::Clone for UserDataTaskListSyncManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskManager {}
impl ::core::clone::Clone for UserDataTaskManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskQueryOptions {}
impl ::core::clone::Clone for UserDataTaskQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskReader {}
impl ::core::clone::Clone for UserDataTaskReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskRecurrenceProperties {}
impl ::core::clone::Clone for UserDataTaskRecurrenceProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskRegenerationProperties {}
impl ::core::clone::Clone for UserDataTaskRegenerationProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct UserDataTaskStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskStore {}
impl ::core::clone::Clone for UserDataTaskStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
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
#[repr(transparent)]
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
