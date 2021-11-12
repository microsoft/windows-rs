#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct INamedResource(pub *mut ::core::ffi::c_void);
pub struct IResourceCandidate(pub *mut ::core::ffi::c_void);
pub struct IResourceCandidate2(pub *mut ::core::ffi::c_void);
pub struct IResourceCandidate3(pub *mut ::core::ffi::c_void);
pub struct IResourceContext(pub *mut ::core::ffi::c_void);
pub struct IResourceContextStatics(pub *mut ::core::ffi::c_void);
pub struct IResourceContextStatics2(pub *mut ::core::ffi::c_void);
pub struct IResourceContextStatics3(pub *mut ::core::ffi::c_void);
pub struct IResourceContextStatics4(pub *mut ::core::ffi::c_void);
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
pub struct IResourceManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IResourceMap(pub *mut ::core::ffi::c_void);
pub struct IResourceQualifier(pub *mut ::core::ffi::c_void);
pub struct NamedResource(i32);
pub struct ResourceCandidate(i32);
pub struct ResourceCandidateKind(i32);
pub struct ResourceCandidateVectorView(i32);
pub struct ResourceContext(i32);
pub struct ResourceContextLanguagesVectorView(i32);
pub struct ResourceLayoutInfo(i32);
pub struct ResourceManager(i32);
pub struct ResourceMap(i32);
pub struct ResourceMapIterator(i32);
pub struct ResourceMapMapView(i32);
pub struct ResourceMapMapViewIterator(i32);
pub struct ResourceQualifier(i32);
pub struct ResourceQualifierMapView(i32);
pub struct ResourceQualifierObservableMap(i32);
pub struct ResourceQualifierPersistence(i32);
pub struct ResourceQualifierVectorView(i32);
