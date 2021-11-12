#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct EyesPose(i32);
pub struct HandJointKind(i32);
pub struct HandMeshObserver(i32);
#[cfg(feature = "Foundation_Numerics")]
pub struct HandMeshVertex(i32);
pub struct HandMeshVertexState(i32);
pub struct HandPose(i32);
pub struct HeadPose(i32);
pub struct IEyesPose(pub *mut ::core::ffi::c_void);
pub struct IEyesPoseStatics(pub *mut ::core::ffi::c_void);
pub struct IHandMeshObserver(pub *mut ::core::ffi::c_void);
pub struct IHandMeshVertexState(pub *mut ::core::ffi::c_void);
pub struct IHandPose(pub *mut ::core::ffi::c_void);
pub struct IHeadPose(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
pub struct JointPose(i32);
pub struct JointPoseAccuracy(i32);
