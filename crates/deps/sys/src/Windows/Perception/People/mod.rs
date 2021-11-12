#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EyesPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: HandJointKind = HandJointKind(0i32);
    pub const Wrist: HandJointKind = HandJointKind(1i32);
    pub const ThumbMetacarpal: HandJointKind = HandJointKind(2i32);
    pub const ThumbProximal: HandJointKind = HandJointKind(3i32);
    pub const ThumbDistal: HandJointKind = HandJointKind(4i32);
    pub const ThumbTip: HandJointKind = HandJointKind(5i32);
    pub const IndexMetacarpal: HandJointKind = HandJointKind(6i32);
    pub const IndexProximal: HandJointKind = HandJointKind(7i32);
    pub const IndexIntermediate: HandJointKind = HandJointKind(8i32);
    pub const IndexDistal: HandJointKind = HandJointKind(9i32);
    pub const IndexTip: HandJointKind = HandJointKind(10i32);
    pub const MiddleMetacarpal: HandJointKind = HandJointKind(11i32);
    pub const MiddleProximal: HandJointKind = HandJointKind(12i32);
    pub const MiddleIntermediate: HandJointKind = HandJointKind(13i32);
    pub const MiddleDistal: HandJointKind = HandJointKind(14i32);
    pub const MiddleTip: HandJointKind = HandJointKind(15i32);
    pub const RingMetacarpal: HandJointKind = HandJointKind(16i32);
    pub const RingProximal: HandJointKind = HandJointKind(17i32);
    pub const RingIntermediate: HandJointKind = HandJointKind(18i32);
    pub const RingDistal: HandJointKind = HandJointKind(19i32);
    pub const RingTip: HandJointKind = HandJointKind(20i32);
    pub const LittleMetacarpal: HandJointKind = HandJointKind(21i32);
    pub const LittleProximal: HandJointKind = HandJointKind(22i32);
    pub const LittleIntermediate: HandJointKind = HandJointKind(23i32);
    pub const LittleDistal: HandJointKind = HandJointKind(24i32);
    pub const LittleTip: HandJointKind = HandJointKind(25i32);
}
#[repr(transparent)]
pub struct HandMeshObserver(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct HandMeshVertex(i32);
#[repr(transparent)]
pub struct HandMeshVertexState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HeadPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEyesPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEyesPoseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandMeshObserver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandMeshVertexState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHeadPose(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct JointPose(i32);
#[repr(transparent)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: JointPoseAccuracy = JointPoseAccuracy(0i32);
    pub const Approximate: JointPoseAccuracy = JointPoseAccuracy(1i32);
}
