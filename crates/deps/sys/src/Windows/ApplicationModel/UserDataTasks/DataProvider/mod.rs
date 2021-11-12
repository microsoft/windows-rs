#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IUserDataTaskDataProviderConnection(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListCompleteTaskRequest(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListCompleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListCreateOrUpdateTaskRequest(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListDeleteTaskRequest(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListDeleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListSkipOccurrenceRequest(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct UserDataTaskDataProviderConnection(i32);
pub struct UserDataTaskDataProviderTriggerDetails(i32);
pub struct UserDataTaskListCompleteTaskRequest(i32);
pub struct UserDataTaskListCompleteTaskRequestEventArgs(i32);
pub struct UserDataTaskListCreateOrUpdateTaskRequest(i32);
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(i32);
pub struct UserDataTaskListDeleteTaskRequest(i32);
pub struct UserDataTaskListDeleteTaskRequestEventArgs(i32);
pub struct UserDataTaskListSkipOccurrenceRequest(i32);
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(i32);
pub struct UserDataTaskListSyncManagerSyncRequest(i32);
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(i32);
