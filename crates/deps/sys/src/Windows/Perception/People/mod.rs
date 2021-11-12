#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EyesPose(pub *mut ::core::ffi::c_void);
pub struct HandJointKind(i32);
#[repr(transparent)]
pub struct HandMeshObserver(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation_Numerics")]
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
pub struct JointPose(i32);
pub struct JointPoseAccuracy(i32);
