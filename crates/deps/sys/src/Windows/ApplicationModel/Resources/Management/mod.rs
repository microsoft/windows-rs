#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IIndexedResourceCandidate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIndexedResourceQualifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceIndexer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceIndexerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceIndexerFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IndexedResourceCandidate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IndexedResourceQualifier(pub *mut ::core::ffi::c_void);
pub struct IndexedResourceType(i32);
#[repr(transparent)]
pub struct ResourceIndexer(pub *mut ::core::ffi::c_void);
pub struct ResourceIndexerContract(i32);
