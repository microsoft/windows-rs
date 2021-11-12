#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialGestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialGestureRecognizerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialHoldCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialHoldCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialHoldStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteraction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionController3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionControllerProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionDetectedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSource3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSource4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceLocation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceState2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialInteractionSourceState3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialManipulationCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialManipulationDelta(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialNavigationCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialNavigationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialNavigationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerInteractionSourcePose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerInteractionSourcePose2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerPose2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerPose3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialPointerPoseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialRecognitionEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialRecognitionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialGestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialGestureSettings(pub u32);
impl SpatialGestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const ManipulationTranslate: Self = Self(8u32);
    pub const NavigationX: Self = Self(16u32);
    pub const NavigationY: Self = Self(32u32);
    pub const NavigationZ: Self = Self(64u32);
    pub const NavigationRailsX: Self = Self(128u32);
    pub const NavigationRailsY: Self = Self(256u32);
    pub const NavigationRailsZ: Self = Self(512u32);
}
impl ::core::marker::Copy for SpatialGestureSettings {}
impl ::core::clone::Clone for SpatialGestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialHoldCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialHoldCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialHoldStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteraction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionControllerProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionPressKind(pub i32);
impl SpatialInteractionPressKind {
    pub const None: Self = Self(0i32);
    pub const Select: Self = Self(1i32);
    pub const Menu: Self = Self(2i32);
    pub const Grasp: Self = Self(3i32);
    pub const Touchpad: Self = Self(4i32);
    pub const Thumbstick: Self = Self(5i32);
}
impl ::core::marker::Copy for SpatialInteractionPressKind {}
impl ::core::clone::Clone for SpatialInteractionPressKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceHandedness {}
impl ::core::clone::Clone for SpatialInteractionSourceHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: Self = Self(0i32);
    pub const Hand: Self = Self(1i32);
    pub const Voice: Self = Self(2i32);
    pub const Controller: Self = Self(3i32);
}
impl ::core::marker::Copy for SpatialInteractionSourceKind {}
impl ::core::clone::Clone for SpatialInteractionSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSourceLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: Self = Self(0i32);
    pub const Approximate: Self = Self(1i32);
}
impl ::core::marker::Copy for SpatialInteractionSourcePositionAccuracy {}
impl ::core::clone::Clone for SpatialInteractionSourcePositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialInteractionSourceProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialManipulationCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialManipulationDelta(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialNavigationCanceledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialNavigationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialNavigationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialPointerInteractionSourcePose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialPointerPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialRecognitionEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialRecognitionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialTappedEventArgs(pub *mut ::core::ffi::c_void);
