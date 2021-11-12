#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(0i32);
    pub const Sharing: SpatialAnchorExportPurpose = SpatialAnchorExportPurpose(1i32);
}
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
#[repr(transparent)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(0i32);
    pub const Started: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(1i32);
    pub const EnumerationCompleted: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(2i32);
    pub const Stopping: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(3i32);
    pub const Stopped: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(4i32);
    pub const Aborted: SpatialEntityWatcherStatus = SpatialEntityWatcherStatus(5i32);
}
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: SpatialLocatability = SpatialLocatability(0i32);
    pub const OrientationOnly: SpatialLocatability = SpatialLocatability(1i32);
    pub const PositionalTrackingActivating: SpatialLocatability = SpatialLocatability(2i32);
    pub const PositionalTrackingActive: SpatialLocatability = SpatialLocatability(3i32);
    pub const PositionalTrackingInhibited: SpatialLocatability = SpatialLocatability(4i32);
}
#[repr(transparent)]
pub struct SpatialLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: SpatialLookDirectionRange = SpatialLookDirectionRange(0i32);
    pub const Omnidirectional: SpatialLookDirectionRange = SpatialLookDirectionRange(1i32);
}
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: SpatialMovementRange = SpatialMovementRange(0i32);
    pub const Bounded: SpatialMovementRange = SpatialMovementRange(1i32);
}
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(0i32);
    pub const Allowed: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(1i32);
    pub const DeniedByUser: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(2i32);
    pub const DeniedBySystem: SpatialPerceptionAccessStatus = SpatialPerceptionAccessStatus(3i32);
}
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct SpatialRay(i32);
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
