#[cfg(feature = "implement_exclusive")]
pub trait IEyesPoseImpl: Sized {
    fn IsCalibrationValid(&self) -> ::windows::core::Result<bool>;
    fn Gaze(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Spatial::SpatialRay>>;
    fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEyesPoseStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::UI::Input::GazeInputAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandMeshObserverImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::UI::Input::Spatial::SpatialInteractionSource>;
    fn TriangleIndexCount(&self) -> ::windows::core::Result<u32>;
    fn VertexCount(&self) -> ::windows::core::Result<u32>;
    fn GetTriangleIndices(&self, indices: &mut [<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetVertexStateForPose(&self, handpose: &::core::option::Option<HandPose>) -> ::windows::core::Result<HandMeshVertexState>;
    fn NeutralPose(&self) -> ::windows::core::Result<HandPose>;
    fn NeutralPoseVersion(&self) -> ::windows::core::Result<i32>;
    fn ModelId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandMeshVertexStateImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::Spatial::SpatialCoordinateSystem>;
    fn GetVertices(&self, vertices: &mut [<HandMeshVertex as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UpdateTimestamp(&self) -> ::windows::core::Result<super::PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHandPoseImpl: Sized {
    fn TryGetJoint(&self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joint: HandJointKind, jointpose: &mut JointPose) -> ::windows::core::Result<bool>;
    fn TryGetJoints(&self, coordinatesystem: &::core::option::Option<super::Spatial::SpatialCoordinateSystem>, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool>;
    fn GetRelativeJoint(&self, joint: HandJointKind, referencejoint: HandJointKind) -> ::windows::core::Result<JointPose>;
    fn GetRelativeJoints(&self, joints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], referencejoints: &[<HandJointKind as ::windows::core::DefaultType>::DefaultType], jointposes: &mut [<JointPose as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHeadPoseImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ForwardDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn UpDirection(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
}
