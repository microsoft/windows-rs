#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ContactDataProviderConnection(i32);
pub struct ContactDataProviderTriggerDetails(i32);
pub struct ContactListCreateOrUpdateContactRequest(i32);
pub struct ContactListCreateOrUpdateContactRequestEventArgs(i32);
pub struct ContactListDeleteContactRequest(i32);
pub struct ContactListDeleteContactRequestEventArgs(i32);
pub struct ContactListServerSearchReadBatchRequest(i32);
pub struct ContactListServerSearchReadBatchRequestEventArgs(i32);
pub struct ContactListSyncManagerSyncRequest(i32);
pub struct ContactListSyncManagerSyncRequestEventArgs(i32);
pub struct IContactDataProviderConnection(pub *mut ::core::ffi::c_void);
pub struct IContactDataProviderConnection2(pub *mut ::core::ffi::c_void);
pub struct IContactDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IContactListCreateOrUpdateContactRequest(pub *mut ::core::ffi::c_void);
pub struct IContactListCreateOrUpdateContactRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IContactListDeleteContactRequest(pub *mut ::core::ffi::c_void);
pub struct IContactListDeleteContactRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IContactListServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
pub struct IContactListServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
pub struct IContactListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
pub struct IContactListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
