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
    pub const None: SpatialGestureSettings = SpatialGestureSettings(0u32);
    pub const Tap: SpatialGestureSettings = SpatialGestureSettings(1u32);
    pub const DoubleTap: SpatialGestureSettings = SpatialGestureSettings(2u32);
    pub const Hold: SpatialGestureSettings = SpatialGestureSettings(4u32);
    pub const ManipulationTranslate: SpatialGestureSettings = SpatialGestureSettings(8u32);
    pub const NavigationX: SpatialGestureSettings = SpatialGestureSettings(16u32);
    pub const NavigationY: SpatialGestureSettings = SpatialGestureSettings(32u32);
    pub const NavigationZ: SpatialGestureSettings = SpatialGestureSettings(64u32);
    pub const NavigationRailsX: SpatialGestureSettings = SpatialGestureSettings(128u32);
    pub const NavigationRailsY: SpatialGestureSettings = SpatialGestureSettings(256u32);
    pub const NavigationRailsZ: SpatialGestureSettings = SpatialGestureSettings(512u32);
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
    pub const None: SpatialInteractionPressKind = SpatialInteractionPressKind(0i32);
    pub const Select: SpatialInteractionPressKind = SpatialInteractionPressKind(1i32);
    pub const Menu: SpatialInteractionPressKind = SpatialInteractionPressKind(2i32);
    pub const Grasp: SpatialInteractionPressKind = SpatialInteractionPressKind(3i32);
    pub const Touchpad: SpatialInteractionPressKind = SpatialInteractionPressKind(4i32);
    pub const Thumbstick: SpatialInteractionPressKind = SpatialInteractionPressKind(5i32);
}
#[repr(transparent)]
pub struct SpatialInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceHandedness(pub i32);
impl SpatialInteractionSourceHandedness {
    pub const Unspecified: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(0i32);
    pub const Left: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(1i32);
    pub const Right: SpatialInteractionSourceHandedness = SpatialInteractionSourceHandedness(2i32);
}
#[repr(transparent)]
pub struct SpatialInteractionSourceKind(pub i32);
impl SpatialInteractionSourceKind {
    pub const Other: SpatialInteractionSourceKind = SpatialInteractionSourceKind(0i32);
    pub const Hand: SpatialInteractionSourceKind = SpatialInteractionSourceKind(1i32);
    pub const Voice: SpatialInteractionSourceKind = SpatialInteractionSourceKind(2i32);
    pub const Controller: SpatialInteractionSourceKind = SpatialInteractionSourceKind(3i32);
}
#[repr(transparent)]
pub struct SpatialInteractionSourceLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourcePositionAccuracy(pub i32);
impl SpatialInteractionSourcePositionAccuracy {
    pub const High: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(0i32);
    pub const Approximate: SpatialInteractionSourcePositionAccuracy = SpatialInteractionSourcePositionAccuracy(1i32);
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
