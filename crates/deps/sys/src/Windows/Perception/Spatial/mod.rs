#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[link(name = "windows")]
extern "system" {}
pub struct ISpatialAnchor(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchor2(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorExportSufficiency(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorExporter(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorExporterStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorStore(pub *mut ::core::ffi::c_void);
pub struct ISpatialAnchorTransferManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialBoundingVolume(pub *mut ::core::ffi::c_void);
pub struct ISpatialBoundingVolumeStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialCoordinateSystem(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntity(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityAddedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityFactory(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityRemovedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityStore(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityStoreStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityUpdatedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISpatialEntityWatcher(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocation(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocation2(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocator(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISpatialLocatorStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
pub struct ISpatialStageFrameOfReferenceStatics(pub *mut ::core::ffi::c_void);
pub struct ISpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
pub struct SpatialAnchor(i32);
pub struct SpatialAnchorExportPurpose(i32);
pub struct SpatialAnchorExportSufficiency(i32);
pub struct SpatialAnchorExporter(i32);
pub struct SpatialAnchorManager(i32);
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(i32);
pub struct SpatialAnchorStore(i32);
pub struct SpatialAnchorTransferManager(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingFrustum(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere(i32);
pub struct SpatialBoundingVolume(i32);
pub struct SpatialCoordinateSystem(i32);
pub struct SpatialEntity(i32);
pub struct SpatialEntityAddedEventArgs(i32);
pub struct SpatialEntityRemovedEventArgs(i32);
pub struct SpatialEntityStore(i32);
pub struct SpatialEntityUpdatedEventArgs(i32);
pub struct SpatialEntityWatcher(i32);
pub struct SpatialEntityWatcherStatus(i32);
pub struct SpatialLocatability(i32);
pub struct SpatialLocation(i32);
pub struct SpatialLocator(i32);
pub struct SpatialLocatorAttachedFrameOfReference(i32);
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(i32);
pub struct SpatialLookDirectionRange(i32);
pub struct SpatialMovementRange(i32);
pub struct SpatialPerceptionAccessStatus(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay(i32);
pub struct SpatialStageFrameOfReference(i32);
pub struct SpatialStationaryFrameOfReference(i32);
