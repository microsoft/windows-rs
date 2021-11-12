#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Perception_Spatial_Preview")]
pub mod Preview;
#[cfg(feature = "Perception_Spatial_Surfaces")]
pub mod Surfaces;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialAnchor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchor {}
impl ::core::clone::Clone for ISpatialAnchor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchor2 {}
impl ::core::clone::Clone for ISpatialAnchor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorExportSufficiency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorExportSufficiency {}
impl ::core::clone::Clone for ISpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorExporter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorExporter {}
impl ::core::clone::Clone for ISpatialAnchorExporter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorExporterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorExporterStatics {}
impl ::core::clone::Clone for ISpatialAnchorExporterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorManagerStatics {}
impl ::core::clone::Clone for ISpatialAnchorManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
impl ::core::clone::Clone for ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorStatics {}
impl ::core::clone::Clone for ISpatialAnchorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorStore {}
impl ::core::clone::Clone for ISpatialAnchorStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialAnchorTransferManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialAnchorTransferManagerStatics {}
impl ::core::clone::Clone for ISpatialAnchorTransferManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialBoundingVolume(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialBoundingVolume {}
impl ::core::clone::Clone for ISpatialBoundingVolume {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialBoundingVolumeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialBoundingVolumeStatics {}
impl ::core::clone::Clone for ISpatialBoundingVolumeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialCoordinateSystem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialCoordinateSystem {}
impl ::core::clone::Clone for ISpatialCoordinateSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntity {}
impl ::core::clone::Clone for ISpatialEntity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityAddedEventArgs {}
impl ::core::clone::Clone for ISpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityFactory {}
impl ::core::clone::Clone for ISpatialEntityFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityRemovedEventArgs {}
impl ::core::clone::Clone for ISpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityStore {}
impl ::core::clone::Clone for ISpatialEntityStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityStoreStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityStoreStatics {}
impl ::core::clone::Clone for ISpatialEntityStoreStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityUpdatedEventArgs {}
impl ::core::clone::Clone for ISpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialEntityWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialEntityWatcher {}
impl ::core::clone::Clone for ISpatialEntityWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocation {}
impl ::core::clone::Clone for ISpatialLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocation2 {}
impl ::core::clone::Clone for ISpatialLocation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocator {}
impl ::core::clone::Clone for ISpatialLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocatorAttachedFrameOfReference {}
impl ::core::clone::Clone for ISpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {}
impl ::core::clone::Clone for ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialLocatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialLocatorStatics {}
impl ::core::clone::Clone for ISpatialLocatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialStageFrameOfReference {}
impl ::core::clone::Clone for ISpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialStageFrameOfReferenceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialStageFrameOfReferenceStatics {}
impl ::core::clone::Clone for ISpatialStageFrameOfReferenceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialStationaryFrameOfReference {}
impl ::core::clone::Clone for ISpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAnchor {}
impl ::core::clone::Clone for SpatialAnchor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchorExportPurpose(pub i32);
impl SpatialAnchorExportPurpose {
    pub const Relocalization: Self = Self(0i32);
    pub const Sharing: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialAnchorExportPurpose {}
impl ::core::clone::Clone for SpatialAnchorExportPurpose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchorExportSufficiency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAnchorExportSufficiency {}
impl ::core::clone::Clone for SpatialAnchorExportSufficiency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchorExporter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAnchorExporter {}
impl ::core::clone::Clone for SpatialAnchorExporter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchorRawCoordinateSystemAdjustedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
impl ::core::clone::Clone for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialAnchorStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialAnchorStore {}
impl ::core::clone::Clone for SpatialAnchorStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingFrustum {
    pub Near: super::super::Foundation::Numerics::Plane,
    pub Far: super::super::Foundation::Numerics::Plane,
    pub Right: super::super::Foundation::Numerics::Plane,
    pub Left: super::super::Foundation::Numerics::Plane,
    pub Top: super::super::Foundation::Numerics::Plane,
    pub Bottom: super::super::Foundation::Numerics::Plane,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingFrustum {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingOrientedBox {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Extents: super::super::Foundation::Numerics::Vector3,
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingOrientedBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialBoundingSphere {
    pub Center: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialBoundingSphere {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialBoundingVolume(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialBoundingVolume {}
impl ::core::clone::Clone for SpatialBoundingVolume {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialCoordinateSystem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialCoordinateSystem {}
impl ::core::clone::Clone for SpatialCoordinateSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntity(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntity {}
impl ::core::clone::Clone for SpatialEntity {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntityAddedEventArgs {}
impl ::core::clone::Clone for SpatialEntityAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntityRemovedEventArgs {}
impl ::core::clone::Clone for SpatialEntityRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityStore(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntityStore {}
impl ::core::clone::Clone for SpatialEntityStore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntityUpdatedEventArgs {}
impl ::core::clone::Clone for SpatialEntityUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialEntityWatcher {}
impl ::core::clone::Clone for SpatialEntityWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialEntityWatcherStatus(pub i32);
impl SpatialEntityWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialEntityWatcherStatus {}
impl ::core::clone::Clone for SpatialEntityWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocatability(pub i32);
impl SpatialLocatability {
    pub const Unavailable: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const PositionalTrackingActivating: Self = Self(2i32);
    pub const PositionalTrackingActive: Self = Self(3i32);
    pub const PositionalTrackingInhibited: Self = Self(4i32);
}
impl ::core::marker::Copy for SpatialLocatability {}
impl ::core::clone::Clone for SpatialLocatability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialLocation {}
impl ::core::clone::Clone for SpatialLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialLocator {}
impl ::core::clone::Clone for SpatialLocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocatorAttachedFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialLocatorAttachedFrameOfReference {}
impl ::core::clone::Clone for SpatialLocatorAttachedFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLocatorPositionalTrackingDeactivatingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
impl ::core::clone::Clone for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialLookDirectionRange(pub i32);
impl SpatialLookDirectionRange {
    pub const ForwardOnly: Self = Self(0i32);
    pub const Omnidirectional: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialLookDirectionRange {}
impl ::core::clone::Clone for SpatialLookDirectionRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialMovementRange(pub i32);
impl SpatialMovementRange {
    pub const NoMovement: Self = Self(0i32);
    pub const Bounded: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialMovementRange {}
impl ::core::clone::Clone for SpatialMovementRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialPerceptionAccessStatus(pub i32);
impl SpatialPerceptionAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialPerceptionAccessStatus {}
impl ::core::clone::Clone for SpatialPerceptionAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct SpatialRay {
    pub Origin: super::super::Foundation::Numerics::Vector3,
    pub Direction: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for SpatialRay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialStageFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialStageFrameOfReference {}
impl ::core::clone::Clone for SpatialStageFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialStationaryFrameOfReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialStationaryFrameOfReference {}
impl ::core::clone::Clone for SpatialStationaryFrameOfReference {
    fn clone(&self) -> Self {
        *self
    }
}
