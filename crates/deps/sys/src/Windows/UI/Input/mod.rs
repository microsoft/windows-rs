#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Input_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking")]
pub mod Inking;
#[cfg(feature = "UI_Input_Preview")]
pub mod Preview;
#[cfg(feature = "UI_Input_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AttachableInputObject(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CrossSlideThresholds(i32);
#[repr(transparent)]
pub struct CrossSlidingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0i32);
    pub const Dragging: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
    pub const SelectSpeedBumping: Self = Self(3i32);
    pub const SpeedBumping: Self = Self(4i32);
    pub const Rearranging: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
#[repr(transparent)]
pub struct DraggingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct EdgeGesture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EdgeGestureEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
#[repr(transparent)]
pub struct GestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0u32);
    pub const Tap: Self = Self(1u32);
    pub const DoubleTap: Self = Self(2u32);
    pub const Hold: Self = Self(4u32);
    pub const HoldWithMouse: Self = Self(8u32);
    pub const RightTap: Self = Self(16u32);
    pub const Drag: Self = Self(32u32);
    pub const ManipulationTranslateX: Self = Self(64u32);
    pub const ManipulationTranslateY: Self = Self(128u32);
    pub const ManipulationTranslateRailsX: Self = Self(256u32);
    pub const ManipulationTranslateRailsY: Self = Self(512u32);
    pub const ManipulationRotate: Self = Self(1024u32);
    pub const ManipulationScale: Self = Self(2048u32);
    pub const ManipulationTranslateInertia: Self = Self(4096u32);
    pub const ManipulationRotateInertia: Self = Self(8192u32);
    pub const ManipulationScaleInertia: Self = Self(16384u32);
    pub const CrossSlide: Self = Self(32768u32);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536u32);
}
#[repr(transparent)]
pub struct HoldingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct IAttachableInputObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAttachableInputObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrossSlidingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICrossSlidingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDraggingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDraggingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEdgeGesture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEdgeGestureEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEdgeGestureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGestureRecognizer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHoldingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHoldingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputActivationListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputActivationListenerActivationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationStartedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMouseWheelParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerPointProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerPointProperties2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerPointStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerPointTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerVisualizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerVisualizationSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerButtonHoldingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerButtonReleasedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerMenuItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContactEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRightTappedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemButtonEventController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemButtonEventControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemFunctionButtonEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemFunctionLockChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITappedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputActivationListener(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputActivationListenerActivationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
#[repr(transparent)]
pub struct KeyboardDeliveryInterceptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct ManipulationDelta(i32);
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct ManipulationVelocities(i32);
#[repr(transparent)]
pub struct MouseWheelParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerPointProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0i32);
    pub const LeftButtonPressed: Self = Self(1i32);
    pub const LeftButtonReleased: Self = Self(2i32);
    pub const RightButtonPressed: Self = Self(3i32);
    pub const RightButtonReleased: Self = Self(4i32);
    pub const MiddleButtonPressed: Self = Self(5i32);
    pub const MiddleButtonReleased: Self = Self(6i32);
    pub const XButton1Pressed: Self = Self(7i32);
    pub const XButton1Released: Self = Self(8i32);
    pub const XButton2Pressed: Self = Self(9i32);
    pub const XButton2Released: Self = Self(10i32);
}
#[repr(transparent)]
pub struct PointerVisualizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerButtonHoldingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerButtonReleasedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerControlAcquiredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerMenuItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerMenuKnownIcon(pub i32);
impl RadialControllerMenuKnownIcon {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
    pub const Ruler: Self = Self(5i32);
    pub const InkColor: Self = Self(6i32);
    pub const InkThickness: Self = Self(7i32);
    pub const PenType: Self = Self(8i32);
}
#[repr(transparent)]
pub struct RadialControllerRotationChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerScreenContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerScreenContactContinuedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerScreenContactEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerScreenContactStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
#[repr(transparent)]
pub struct RightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemButtonEventController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemFunctionButtonEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemFunctionLockChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TappedEventArgs(pub *mut ::core::ffi::c_void);
