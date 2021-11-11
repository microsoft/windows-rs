#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ISpatialSurfaceInfo();
    fn ISpatialSurfaceMesh();
    fn ISpatialSurfaceMeshBuffer();
    fn ISpatialSurfaceMeshOptions();
    fn ISpatialSurfaceMeshOptionsStatics();
    fn ISpatialSurfaceObserver();
    fn ISpatialSurfaceObserverStatics();
    fn ISpatialSurfaceObserverStatics2();
    fn SpatialSurfaceInfo();
    fn SpatialSurfaceMesh();
    fn SpatialSurfaceMeshBuffer();
    fn SpatialSurfaceMeshOptions();
    fn SpatialSurfaceObserver();
}
