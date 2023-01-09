impl ::core::cmp::PartialEq for SpatialAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchor {}
impl ::core::fmt::Debug for SpatialAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchor").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialAnchorExportPurpose {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialAnchorExportPurpose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportPurpose").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExportSufficiency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExportSufficiency {}
impl ::core::fmt::Debug for SpatialAnchorExportSufficiency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExportSufficiency").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorExporter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorExporter {}
impl ::core::fmt::Debug for SpatialAnchorExporter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorExporter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {}
impl ::core::fmt::Debug for SpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorRawCoordinateSystemAdjustedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAnchorStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialAnchorStore {}
impl ::core::fmt::Debug for SpatialAnchorStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAnchorStore").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Extents == other.Extents
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingBox").field("Center", &self.Center).field("Extents", &self.Extents).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingFrustum {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingFrustum {
    fn eq(&self, other: &Self) -> bool {
        self.Near == other.Near && self.Far == other.Far && self.Right == other.Right && self.Left == other.Left && self.Top == other.Top && self.Bottom == other.Bottom
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingFrustum {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingFrustum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingFrustum").field("Near", &self.Near).field("Far", &self.Far).field("Right", &self.Right).field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingOrientedBox {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingOrientedBox {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Extents == other.Extents && self.Orientation == other.Orientation
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingOrientedBox {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingOrientedBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingOrientedBox").field("Center", &self.Center).field("Extents", &self.Extents).field("Orientation", &self.Orientation).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialBoundingSphere {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialBoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        self.Center == other.Center && self.Radius == other.Radius
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialBoundingSphere {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialBoundingSphere {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialBoundingSphere").field("Center", &self.Center).field("Radius", &self.Radius).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialBoundingVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialBoundingVolume {}
impl ::core::fmt::Debug for SpatialBoundingVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialBoundingVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialCoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialCoordinateSystem {}
impl ::core::fmt::Debug for SpatialCoordinateSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialCoordinateSystem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntity {}
impl ::core::fmt::Debug for SpatialEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntityAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityAddedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityAddedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntityRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityRemovedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntityStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityStore {}
impl ::core::fmt::Debug for SpatialEntityStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntityUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityUpdatedEventArgs {}
impl ::core::fmt::Debug for SpatialEntityUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialEntityWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialEntityWatcher {}
impl ::core::fmt::Debug for SpatialEntityWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcher").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialEntityWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialEntityWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialEntityWatcherStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialLocatability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialLocatability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatability").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocation {}
impl ::core::fmt::Debug for SpatialLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocator {}
impl ::core::fmt::Debug for SpatialLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorAttachedFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorAttachedFrameOfReference {}
impl ::core::fmt::Debug for SpatialLocatorAttachedFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorAttachedFrameOfReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialLocatorPositionalTrackingDeactivatingEventArgs {}
impl ::core::fmt::Debug for SpatialLocatorPositionalTrackingDeactivatingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLocatorPositionalTrackingDeactivatingEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialLookDirectionRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialLookDirectionRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialLookDirectionRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialMovementRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialMovementRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialMovementRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for SpatialPerceptionAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SpatialPerceptionAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialPerceptionAccessStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for SpatialRay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for SpatialRay {
    fn eq(&self, other: &Self) -> bool {
        self.Origin == other.Origin && self.Direction == other.Direction
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for SpatialRay {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for SpatialRay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialRay").field("Origin", &self.Origin).field("Direction", &self.Direction).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialStageFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStageFrameOfReference {}
impl ::core::fmt::Debug for SpatialStageFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStageFrameOfReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SpatialStationaryFrameOfReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialStationaryFrameOfReference {}
impl ::core::fmt::Debug for SpatialStationaryFrameOfReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialStationaryFrameOfReference").field(&self.0).finish()
    }
}
