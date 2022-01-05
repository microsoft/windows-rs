#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropFrameOfReferencePreviewImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn NodeId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CoordinateSystemToNodeTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropPreviewStaticsImpl: Sized {
    fn CreateCoordinateSystemForNode(&self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPosition(&self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPositionAndOrientation(&self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateLocatorForNode(&self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropPreviewStatics2Impl: Sized {
    fn TryCreateFrameOfReference(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPosition(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPositionAndOrientation(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
}
