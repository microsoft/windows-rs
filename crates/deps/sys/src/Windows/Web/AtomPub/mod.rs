#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AtomPubClient(i32);
pub struct IAtomPubClient(pub *mut ::core::ffi::c_void);
pub struct IAtomPubClientFactory(pub *mut ::core::ffi::c_void);
pub struct IResourceCollection(pub *mut ::core::ffi::c_void);
pub struct IServiceDocument(pub *mut ::core::ffi::c_void);
pub struct IWorkspace(pub *mut ::core::ffi::c_void);
pub struct ResourceCollection(i32);
pub struct ServiceDocument(i32);
pub struct Workspace(i32);
