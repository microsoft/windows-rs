#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct INamedResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceCandidate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceCandidate2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceCandidate3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceContextStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceContextStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceContextStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceContextStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceQualifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NamedResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceCandidate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ResourceCandidateVectorView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceContextLanguagesVectorView(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ResourceLayoutInfo(i32);
#[repr(transparent)]
pub struct ResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceMapIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceMapMapView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceMapMapViewIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceQualifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceQualifierMapView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceQualifierObservableMap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
#[repr(transparent)]
pub struct ResourceQualifierVectorView(pub *mut ::core::ffi::c_void);
