#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContactDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactDataProviderConnection {}
impl ::core::clone::Clone for ContactDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactDataProviderTriggerDetails {}
impl ::core::clone::Clone for ContactDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListCreateOrUpdateContactRequest {}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListCreateOrUpdateContactRequestEventArgs {}
impl ::core::clone::Clone for ContactListCreateOrUpdateContactRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListDeleteContactRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListDeleteContactRequest {}
impl ::core::clone::Clone for ContactListDeleteContactRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListDeleteContactRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListDeleteContactRequestEventArgs {}
impl ::core::clone::Clone for ContactListDeleteContactRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListServerSearchReadBatchRequest {}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListServerSearchReadBatchRequestEventArgs {}
impl ::core::clone::Clone for ContactListServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListSyncManagerSyncRequest {}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactListSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for ContactListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactDataProviderConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactDataProviderConnection {}
impl ::core::clone::Clone for IContactDataProviderConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactDataProviderConnection2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactDataProviderConnection2 {}
impl ::core::clone::Clone for IContactDataProviderConnection2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactDataProviderTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactDataProviderTriggerDetails {}
impl ::core::clone::Clone for IContactDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListCreateOrUpdateContactRequest {}
impl ::core::clone::Clone for IContactListCreateOrUpdateContactRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListCreateOrUpdateContactRequestEventArgs {}
impl ::core::clone::Clone for IContactListCreateOrUpdateContactRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListDeleteContactRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListDeleteContactRequest {}
impl ::core::clone::Clone for IContactListDeleteContactRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListDeleteContactRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListDeleteContactRequestEventArgs {}
impl ::core::clone::Clone for IContactListDeleteContactRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListServerSearchReadBatchRequest {}
impl ::core::clone::Clone for IContactListServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListServerSearchReadBatchRequestEventArgs {}
impl ::core::clone::Clone for IContactListServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListSyncManagerSyncRequest {}
impl ::core::clone::Clone for IContactListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactListSyncManagerSyncRequestEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactListSyncManagerSyncRequestEventArgs {}
impl ::core::clone::Clone for IContactListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
