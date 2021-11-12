#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct UserDataTaskDaysOfWeek(i32);
#[repr(C)]
pub struct UserDataTaskDetailsKind(i32);
#[repr(C)]
pub struct UserDataTaskKind(i32);
#[repr(transparent)]
pub struct UserDataTaskList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListLimitedWriteOperations(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskListOtherAppReadAccess(i32);
#[repr(C)]
pub struct UserDataTaskListOtherAppWriteAccess(i32);
#[repr(transparent)]
pub struct UserDataTaskListSyncManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskListSyncStatus(i32);
#[repr(transparent)]
pub struct UserDataTaskManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskPriority(i32);
#[repr(C)]
pub struct UserDataTaskQueryKind(i32);
#[repr(transparent)]
pub struct UserDataTaskQueryOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskQuerySortProperty(i32);
#[repr(transparent)]
pub struct UserDataTaskReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskRecurrenceProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskRecurrenceUnit(i32);
#[repr(transparent)]
pub struct UserDataTaskRegenerationProperties(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskRegenerationUnit(i32);
#[repr(C)]
pub struct UserDataTaskSensitivity(i32);
#[repr(transparent)]
pub struct UserDataTaskStore(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UserDataTaskStoreAccessType(i32);
#[repr(C)]
pub struct UserDataTaskWeekOfMonth(i32);
