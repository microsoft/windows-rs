#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}
pub struct IUserDataTask(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskBatch(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskList(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskManager(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskReader(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskStore(pub *mut ::core::ffi::c_void);
pub struct UserDataTask(i32);
pub struct UserDataTaskBatch(i32);
pub struct UserDataTaskDaysOfWeek(i32);
pub struct UserDataTaskDetailsKind(i32);
pub struct UserDataTaskKind(i32);
pub struct UserDataTaskList(i32);
pub struct UserDataTaskListLimitedWriteOperations(i32);
pub struct UserDataTaskListOtherAppReadAccess(i32);
pub struct UserDataTaskListOtherAppWriteAccess(i32);
pub struct UserDataTaskListSyncManager(i32);
pub struct UserDataTaskListSyncStatus(i32);
pub struct UserDataTaskManager(i32);
pub struct UserDataTaskPriority(i32);
pub struct UserDataTaskQueryKind(i32);
pub struct UserDataTaskQueryOptions(i32);
pub struct UserDataTaskQuerySortProperty(i32);
pub struct UserDataTaskReader(i32);
pub struct UserDataTaskRecurrenceProperties(i32);
pub struct UserDataTaskRecurrenceUnit(i32);
pub struct UserDataTaskRegenerationProperties(i32);
pub struct UserDataTaskRegenerationUnit(i32);
pub struct UserDataTaskSensitivity(i32);
pub struct UserDataTaskStore(i32);
pub struct UserDataTaskStoreAccessType(i32);
pub struct UserDataTaskWeekOfMonth(i32);
