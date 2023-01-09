impl ::core::cmp::PartialEq for EyesPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EyesPose {}
impl ::core::fmt::Debug for EyesPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EyesPose").field(&self.0).finish()
    }
}
impl ::core::default::Default for HandJointKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HandJointKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandJointKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HandMeshObserver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HandMeshObserver {}
impl ::core::fmt::Debug for HandMeshObserver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandMeshObserver").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for HandMeshVertex {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for HandMeshVertex {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Normal == other.Normal
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for HandMeshVertex {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for HandMeshVertex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HandMeshVertex").field("Position", &self.Position).field("Normal", &self.Normal).finish()
    }
}
impl ::core::cmp::PartialEq for HandMeshVertexState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HandMeshVertexState {}
impl ::core::fmt::Debug for HandMeshVertexState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandMeshVertexState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HandPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HandPose {}
impl ::core::fmt::Debug for HandPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandPose").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HeadPose {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HeadPose {}
impl ::core::fmt::Debug for HeadPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HeadPose").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for JointPose {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for JointPose {
    fn eq(&self, other: &Self) -> bool {
        self.Orientation == other.Orientation && self.Position == other.Position && self.Radius == other.Radius && self.Accuracy == other.Accuracy
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for JointPose {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for JointPose {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JointPose").field("Orientation", &self.Orientation).field("Position", &self.Position).field("Radius", &self.Radius).field("Accuracy", &self.Accuracy).finish()
    }
}
impl ::core::default::Default for JointPoseAccuracy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JointPoseAccuracy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JointPoseAccuracy").field(&self.0).finish()
    }
}
