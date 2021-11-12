#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialSurfaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialSurfaceMesh(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialSurfaceObserver(pub *mut ::core::ffi::c_void);
