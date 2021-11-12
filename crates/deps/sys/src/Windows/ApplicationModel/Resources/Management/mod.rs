#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IIndexedResourceCandidate(pub *mut ::core::ffi::c_void);
pub struct IIndexedResourceQualifier(pub *mut ::core::ffi::c_void);
pub struct IResourceIndexer(pub *mut ::core::ffi::c_void);
pub struct IResourceIndexerFactory(pub *mut ::core::ffi::c_void);
pub struct IResourceIndexerFactory2(pub *mut ::core::ffi::c_void);
pub struct IndexedResourceCandidate(i32);
pub struct IndexedResourceQualifier(i32);
pub struct IndexedResourceType(i32);
pub struct ResourceIndexer(i32);
pub struct ResourceIndexerContract(i32);
