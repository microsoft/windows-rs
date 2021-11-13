#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceInfo {}
impl ::core::clone::Clone for ISpatialSurfaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceMesh {}
impl ::core::clone::Clone for ISpatialSurfaceMesh {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceMeshBuffer {}
impl ::core::clone::Clone for ISpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceMeshOptions {}
impl ::core::clone::Clone for ISpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceMeshOptionsStatics {}
impl ::core::clone::Clone for ISpatialSurfaceMeshOptionsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceObserver {}
impl ::core::clone::Clone for ISpatialSurfaceObserver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceObserverStatics {}
impl ::core::clone::Clone for ISpatialSurfaceObserverStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialSurfaceObserverStatics2 {}
impl ::core::clone::Clone for ISpatialSurfaceObserverStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialSurfaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialSurfaceInfo {}
impl ::core::clone::Clone for SpatialSurfaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialSurfaceMesh(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialSurfaceMesh {}
impl ::core::clone::Clone for SpatialSurfaceMesh {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialSurfaceMeshBuffer {}
impl ::core::clone::Clone for SpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialSurfaceMeshOptions {}
impl ::core::clone::Clone for SpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialSurfaceObserver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialSurfaceObserver {}
impl ::core::clone::Clone for SpatialSurfaceObserver {
    fn clone(&self) -> Self {
        *self
    }
}
