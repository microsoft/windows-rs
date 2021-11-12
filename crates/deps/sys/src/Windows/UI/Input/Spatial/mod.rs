#![allow(non_snake_case, non_camel_case_types)]
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
pub struct SpatialGestureSettings(i32);
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
pub struct SpatialInteractionPressKind(i32);
#[repr(transparent)]
pub struct SpatialInteractionSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialInteractionSourceEventArgs(pub *mut ::core::ffi::c_void);
pub struct SpatialInteractionSourceHandedness(i32);
pub struct SpatialInteractionSourceKind(i32);
#[repr(transparent)]
pub struct SpatialInteractionSourceLocation(pub *mut ::core::ffi::c_void);
pub struct SpatialInteractionSourcePositionAccuracy(i32);
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
