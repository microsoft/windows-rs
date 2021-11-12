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
    pub const Started: CrossSlidingState = CrossSlidingState(0i32);
    pub const Dragging: CrossSlidingState = CrossSlidingState(1i32);
    pub const Selecting: CrossSlidingState = CrossSlidingState(2i32);
    pub const SelectSpeedBumping: CrossSlidingState = CrossSlidingState(3i32);
    pub const SpeedBumping: CrossSlidingState = CrossSlidingState(4i32);
    pub const Rearranging: CrossSlidingState = CrossSlidingState(5i32);
    pub const Completed: CrossSlidingState = CrossSlidingState(6i32);
}
#[repr(transparent)]
pub struct DraggingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: DraggingState = DraggingState(0i32);
    pub const Continuing: DraggingState = DraggingState(1i32);
    pub const Completed: DraggingState = DraggingState(2i32);
}
#[repr(transparent)]
pub struct EdgeGesture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EdgeGestureEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: EdgeGestureKind = EdgeGestureKind(0i32);
    pub const Keyboard: EdgeGestureKind = EdgeGestureKind(1i32);
    pub const Mouse: EdgeGestureKind = EdgeGestureKind(2i32);
}
#[repr(transparent)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: GazeInputAccessStatus = GazeInputAccessStatus(0i32);
    pub const Allowed: GazeInputAccessStatus = GazeInputAccessStatus(1i32);
    pub const DeniedByUser: GazeInputAccessStatus = GazeInputAccessStatus(2i32);
    pub const DeniedBySystem: GazeInputAccessStatus = GazeInputAccessStatus(3i32);
}
#[repr(transparent)]
pub struct GestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: GestureSettings = GestureSettings(0u32);
    pub const Tap: GestureSettings = GestureSettings(1u32);
    pub const DoubleTap: GestureSettings = GestureSettings(2u32);
    pub const Hold: GestureSettings = GestureSettings(4u32);
    pub const HoldWithMouse: GestureSettings = GestureSettings(8u32);
    pub const RightTap: GestureSettings = GestureSettings(16u32);
    pub const Drag: GestureSettings = GestureSettings(32u32);
    pub const ManipulationTranslateX: GestureSettings = GestureSettings(64u32);
    pub const ManipulationTranslateY: GestureSettings = GestureSettings(128u32);
    pub const ManipulationTranslateRailsX: GestureSettings = GestureSettings(256u32);
    pub const ManipulationTranslateRailsY: GestureSettings = GestureSettings(512u32);
    pub const ManipulationRotate: GestureSettings = GestureSettings(1024u32);
    pub const ManipulationScale: GestureSettings = GestureSettings(2048u32);
    pub const ManipulationTranslateInertia: GestureSettings = GestureSettings(4096u32);
    pub const ManipulationRotateInertia: GestureSettings = GestureSettings(8192u32);
    pub const ManipulationScaleInertia: GestureSettings = GestureSettings(16384u32);
    pub const CrossSlide: GestureSettings = GestureSettings(32768u32);
    pub const ManipulationMultipleFingerPanning: GestureSettings = GestureSettings(65536u32);
}
#[repr(transparent)]
pub struct HoldingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: HoldingState = HoldingState(0i32);
    pub const Completed: HoldingState = HoldingState(1i32);
    pub const Canceled: HoldingState = HoldingState(2i32);
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
    pub const None: InputActivationState = InputActivationState(0i32);
    pub const Deactivated: InputActivationState = InputActivationState(1i32);
    pub const ActivatedNotForeground: InputActivationState = InputActivationState(2i32);
    pub const ActivatedInForeground: InputActivationState = InputActivationState(3i32);
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
    pub const Other: PointerUpdateKind = PointerUpdateKind(0i32);
    pub const LeftButtonPressed: PointerUpdateKind = PointerUpdateKind(1i32);
    pub const LeftButtonReleased: PointerUpdateKind = PointerUpdateKind(2i32);
    pub const RightButtonPressed: PointerUpdateKind = PointerUpdateKind(3i32);
    pub const RightButtonReleased: PointerUpdateKind = PointerUpdateKind(4i32);
    pub const MiddleButtonPressed: PointerUpdateKind = PointerUpdateKind(5i32);
    pub const MiddleButtonReleased: PointerUpdateKind = PointerUpdateKind(6i32);
    pub const XButton1Pressed: PointerUpdateKind = PointerUpdateKind(7i32);
    pub const XButton1Released: PointerUpdateKind = PointerUpdateKind(8i32);
    pub const XButton2Pressed: PointerUpdateKind = PointerUpdateKind(9i32);
    pub const XButton2Released: PointerUpdateKind = PointerUpdateKind(10i32);
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
    pub const Scroll: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(0i32);
    pub const Zoom: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(1i32);
    pub const UndoRedo: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(2i32);
    pub const Volume: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(3i32);
    pub const NextPreviousTrack: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(4i32);
    pub const Ruler: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(5i32);
    pub const InkColor: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(6i32);
    pub const InkThickness: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(7i32);
    pub const PenType: RadialControllerMenuKnownIcon = RadialControllerMenuKnownIcon(8i32);
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
    pub const Scroll: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(0i32);
    pub const Zoom: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(1i32);
    pub const UndoRedo: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(2i32);
    pub const Volume: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(3i32);
    pub const NextPreviousTrack: RadialControllerSystemMenuItemKind = RadialControllerSystemMenuItemKind(4i32);
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
