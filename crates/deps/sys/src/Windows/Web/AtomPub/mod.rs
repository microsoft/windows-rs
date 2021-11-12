#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AtomPubClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AtomPubClient {}
impl ::core::clone::Clone for AtomPubClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAtomPubClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAtomPubClient {}
impl ::core::clone::Clone for IAtomPubClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAtomPubClientFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAtomPubClientFactory {}
impl ::core::clone::Clone for IAtomPubClientFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceCollection {}
impl ::core::clone::Clone for IResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IServiceDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IServiceDocument {}
impl ::core::clone::Clone for IServiceDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWorkspace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWorkspace {}
impl ::core::clone::Clone for IWorkspace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceCollection {}
impl ::core::clone::Clone for ResourceCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ServiceDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ServiceDocument {}
impl ::core::clone::Clone for ServiceDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Workspace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Workspace {}
impl ::core::clone::Clone for Workspace {
    fn clone(&self) -> Self {
        *self
    }
}
