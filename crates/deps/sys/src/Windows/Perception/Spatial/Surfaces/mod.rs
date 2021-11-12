#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISpatialSurfaceInfo(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceMesh(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceMeshBuffer(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceMeshOptions(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceMeshOptionsStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceObserver(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceObserverStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialSurfaceObserverStatics2(pub *mut ::core::ffi::c_void);
pub struct SpatialSurfaceInfo(i32);
pub struct SpatialSurfaceMesh(i32);
pub struct SpatialSurfaceMeshBuffer(i32);
pub struct SpatialSurfaceMeshOptions(i32);
pub struct SpatialSurfaceObserver(i32);
