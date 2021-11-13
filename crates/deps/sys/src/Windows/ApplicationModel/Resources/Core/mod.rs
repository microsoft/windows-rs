#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INamedResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INamedResource {}
impl ::core::clone::Clone for INamedResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceCandidate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceCandidate {}
impl ::core::clone::Clone for IResourceCandidate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceCandidate2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceCandidate2 {}
impl ::core::clone::Clone for IResourceCandidate2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceCandidate3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceCandidate3 {}
impl ::core::clone::Clone for IResourceCandidate3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceContext {}
impl ::core::clone::Clone for IResourceContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceContextStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceContextStatics {}
impl ::core::clone::Clone for IResourceContextStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceContextStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceContextStatics2 {}
impl ::core::clone::Clone for IResourceContextStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceContextStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceContextStatics3 {}
impl ::core::clone::Clone for IResourceContextStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceContextStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceContextStatics4 {}
impl ::core::clone::Clone for IResourceContextStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManager {}
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManager2 {}
impl ::core::clone::Clone for IResourceManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManagerStatics {}
impl ::core::clone::Clone for IResourceManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceMap {}
impl ::core::clone::Clone for IResourceMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceQualifier {}
impl ::core::clone::Clone for IResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NamedResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NamedResource {}
impl ::core::clone::Clone for NamedResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceCandidate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceCandidate {}
impl ::core::clone::Clone for ResourceCandidate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceCandidateVectorView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceCandidateVectorView {}
impl ::core::clone::Clone for ResourceCandidateVectorView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceContext {}
impl ::core::clone::Clone for ResourceContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceContextLanguagesVectorView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceContextLanguagesVectorView {}
impl ::core::clone::Clone for ResourceContextLanguagesVectorView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ::core::marker::Copy for ResourceLayoutInfo {}
impl ::core::clone::Clone for ResourceLayoutInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceManager {}
impl ::core::clone::Clone for ResourceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceMap {}
impl ::core::clone::Clone for ResourceMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceMapIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceMapIterator {}
impl ::core::clone::Clone for ResourceMapIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceMapMapView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceMapMapView {}
impl ::core::clone::Clone for ResourceMapMapView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceMapMapViewIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceMapMapViewIterator {}
impl ::core::clone::Clone for ResourceMapMapViewIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceQualifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceQualifier {}
impl ::core::clone::Clone for ResourceQualifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceQualifierMapView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceQualifierMapView {}
impl ::core::clone::Clone for ResourceQualifierMapView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceQualifierObservableMap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceQualifierObservableMap {}
impl ::core::clone::Clone for ResourceQualifierObservableMap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl ::core::marker::Copy for ResourceQualifierPersistence {}
impl ::core::clone::Clone for ResourceQualifierPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResourceQualifierVectorView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ResourceQualifierVectorView {}
impl ::core::clone::Clone for ResourceQualifierVectorView {
    fn clone(&self) -> Self {
        *self
    }
}
