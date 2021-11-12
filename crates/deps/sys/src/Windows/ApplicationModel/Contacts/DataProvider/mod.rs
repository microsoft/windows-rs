#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContactDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListDeleteContactRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListDeleteContactRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactDataProviderConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactDataProviderConnection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListDeleteContactRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListDeleteContactRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
