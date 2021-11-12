#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserDataTaskDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskDataProviderConnection {}
impl ::core::clone::Clone for IUserDataTaskDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskDataProviderTriggerDetails {}
impl ::core::clone::Clone for IUserDataTaskDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListCompleteTaskRequest {}
impl ::core::clone::Clone for IUserDataTaskListCompleteTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListCompleteTaskRequestEventArgs {}
impl ::core::clone::Clone for IUserDataTaskListCompleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListCreateOrUpdateTaskRequest {}
impl ::core::clone::Clone for IUserDataTaskListCreateOrUpdateTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
impl ::core::clone::Clone for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListDeleteTaskRequest {}
impl ::core::clone::Clone for IUserDataTaskListDeleteTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListDeleteTaskRequestEventArgs {}
impl ::core::clone::Clone for IUserDataTaskListDeleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListSkipOccurrenceRequest {}
impl ::core::clone::Clone for IUserDataTaskListSkipOccurrenceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListSkipOccurrenceRequestEventArgs {}
impl ::core::clone::Clone for IUserDataTaskListSkipOccurrenceRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListSyncManagerSyncRequest {}
impl ::core::clone::Clone for IUserDataTaskListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserDataTaskListSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for IUserDataTaskListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskDataProviderConnection {}
impl ::core::clone::Clone for UserDataTaskDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskDataProviderTriggerDetails {}
impl ::core::clone::Clone for UserDataTaskDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListCompleteTaskRequest {}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListCompleteTaskRequestEventArgs {}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListCreateOrUpdateTaskRequest {}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListDeleteTaskRequest {}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListDeleteTaskRequestEventArgs {}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListSkipOccurrenceRequest {}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListSkipOccurrenceRequestEventArgs {}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListSyncManagerSyncRequest {}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserDataTaskListSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
