#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AtomPubClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAtomPubClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAtomPubClientFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IServiceDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWorkspace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ServiceDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Workspace(pub *mut ::core::ffi::c_void);
