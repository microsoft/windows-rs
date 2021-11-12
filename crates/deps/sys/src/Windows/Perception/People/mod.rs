#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EyesPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EyesPose {}
impl ::core::clone::Clone for EyesPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandJointKind(pub i32);
impl HandJointKind {
    pub const Palm: Self = Self(0i32);
    pub const Wrist: Self = Self(1i32);
    pub const ThumbMetacarpal: Self = Self(2i32);
    pub const ThumbProximal: Self = Self(3i32);
    pub const ThumbDistal: Self = Self(4i32);
    pub const ThumbTip: Self = Self(5i32);
    pub const IndexMetacarpal: Self = Self(6i32);
    pub const IndexProximal: Self = Self(7i32);
    pub const IndexIntermediate: Self = Self(8i32);
    pub const IndexDistal: Self = Self(9i32);
    pub const IndexTip: Self = Self(10i32);
    pub const MiddleMetacarpal: Self = Self(11i32);
    pub const MiddleProximal: Self = Self(12i32);
    pub const MiddleIntermediate: Self = Self(13i32);
    pub const MiddleDistal: Self = Self(14i32);
    pub const MiddleTip: Self = Self(15i32);
    pub const RingMetacarpal: Self = Self(16i32);
    pub const RingProximal: Self = Self(17i32);
    pub const RingIntermediate: Self = Self(18i32);
    pub const RingDistal: Self = Self(19i32);
    pub const RingTip: Self = Self(20i32);
    pub const LittleMetacarpal: Self = Self(21i32);
    pub const LittleProximal: Self = Self(22i32);
    pub const LittleIntermediate: Self = Self(23i32);
    pub const LittleDistal: Self = Self(24i32);
    pub const LittleTip: Self = Self(25i32);
}
impl ::core::marker::Copy for HandJointKind {}
impl ::core::clone::Clone for HandJointKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandMeshObserver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandMeshObserver {}
impl ::core::clone::Clone for HandMeshObserver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct HandMeshVertex {
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Normal: super::super::Foundation::Numerics::Vector3,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HandMeshVertex {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HandMeshVertex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandMeshVertexState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandMeshVertexState {}
impl ::core::clone::Clone for HandMeshVertexState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandPose {}
impl ::core::clone::Clone for HandPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HeadPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HeadPose {}
impl ::core::clone::Clone for HeadPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEyesPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEyesPose {}
impl ::core::clone::Clone for IEyesPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEyesPoseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEyesPoseStatics {}
impl ::core::clone::Clone for IEyesPoseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandMeshObserver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandMeshObserver {}
impl ::core::clone::Clone for IHandMeshObserver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandMeshVertexState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandMeshVertexState {}
impl ::core::clone::Clone for IHandMeshVertexState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandPose {}
impl ::core::clone::Clone for IHandPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHeadPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHeadPose {}
impl ::core::clone::Clone for IHeadPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct JointPose {
    pub Orientation: super::super::Foundation::Numerics::Quaternion,
    pub Position: super::super::Foundation::Numerics::Vector3,
    pub Radius: f32,
    pub Accuracy: JointPoseAccuracy,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for JointPose {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for JointPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JointPoseAccuracy(pub i32);
impl JointPoseAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl ::core::marker::Copy for JointPoseAccuracy {}
impl ::core::clone::Clone for JointPoseAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
