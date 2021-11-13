#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IIndexedResourceCandidate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIndexedResourceCandidate {}
impl ::core::clone::Clone for IIndexedResourceCandidate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIndexedResourceQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIndexedResourceQualifier {}
impl ::core::clone::Clone for IIndexedResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceIndexer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceIndexer {}
impl ::core::clone::Clone for IResourceIndexer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceIndexerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceIndexerFactory {}
impl ::core::clone::Clone for IResourceIndexerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceIndexerFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceIndexerFactory2 {}
impl ::core::clone::Clone for IResourceIndexerFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexedResourceCandidate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IndexedResourceCandidate {}
impl ::core::clone::Clone for IndexedResourceCandidate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexedResourceQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IndexedResourceQualifier {}
impl ::core::clone::Clone for IndexedResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IndexedResourceType(pub i32);
impl IndexedResourceType {
    pub const String: Self = Self(0i32);
    pub const Path: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for IndexedResourceType {}
impl ::core::clone::Clone for IndexedResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceIndexer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceIndexer {}
impl ::core::clone::Clone for ResourceIndexer {
    fn clone(&self) -> Self {
        *self
    }
}
