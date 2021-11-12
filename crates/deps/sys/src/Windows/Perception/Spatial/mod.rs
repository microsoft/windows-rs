#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialAnchor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorExportSufficiency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorExporter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorExporterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialAnchorTransferManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialBoundingVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialBoundingVolumeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialCoordinateSystem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityStoreStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialEntityWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialLocatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialStageFrameOfReferenceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAnchor(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpatialAnchorExportPurpose(i32);
#[repr(transparent)]
pub struct SpatialAnchorExportSufficiency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAnchorExporter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialAnchorStore(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialBoundingBox(i32);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialBoundingFrustum(i32);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialBoundingOrientedBox(i32);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialBoundingSphere(i32);
#[repr(transparent)]
pub struct SpatialBoundingVolume(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialCoordinateSystem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntityAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntityRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntityStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntityUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialEntityWatcher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpatialEntityWatcherStatus(i32);
#[repr(C)]
pub struct SpatialLocatability(i32);
#[repr(transparent)]
pub struct SpatialLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SpatialLookDirectionRange(i32);
#[repr(C)]
pub struct SpatialMovementRange(i32);
#[repr(C)]
pub struct SpatialPerceptionAccessStatus(i32);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialRay(i32);
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
