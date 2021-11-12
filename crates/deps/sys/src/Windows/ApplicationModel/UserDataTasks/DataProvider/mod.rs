#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataTaskDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
