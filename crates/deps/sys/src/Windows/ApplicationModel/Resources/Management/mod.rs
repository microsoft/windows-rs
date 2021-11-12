#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: IndexedResourceType = IndexedResourceType(0i32);
    pub const Path: IndexedResourceType = IndexedResourceType(1i32);
    pub const EmbeddedData: IndexedResourceType = IndexedResourceType(2i32);
}
#[repr(transparent)]
pub struct ResourceIndexer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ResourceIndexerContract(i32);
