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
impl ::core::marker::Copy for AttachableInputObject {}
impl ::core::clone::Clone for AttachableInputObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl ::core::marker::Copy for CrossSlideThresholds {}
impl ::core::clone::Clone for CrossSlideThresholds {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CrossSlidingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CrossSlidingEventArgs {}
impl ::core::clone::Clone for CrossSlidingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CrossSlidingState {}
impl ::core::clone::Clone for CrossSlidingState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DraggingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DraggingEventArgs {}
impl ::core::clone::Clone for DraggingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0i32);
    pub const Continuing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for DraggingState {}
impl ::core::clone::Clone for DraggingState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EdgeGesture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EdgeGesture {}
impl ::core::clone::Clone for EdgeGesture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EdgeGestureEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EdgeGestureEventArgs {}
impl ::core::clone::Clone for EdgeGestureEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EdgeGestureKind(pub i32);
impl EdgeGestureKind {
    pub const Touch: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
}
impl ::core::marker::Copy for EdgeGestureKind {}
impl ::core::clone::Clone for EdgeGestureKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GazeInputAccessStatus(pub i32);
impl GazeInputAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for GazeInputAccessStatus {}
impl ::core::clone::Clone for GazeInputAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GestureRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GestureRecognizer {}
impl ::core::clone::Clone for GestureRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for GestureSettings {}
impl ::core::clone::Clone for GestureSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HoldingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HoldingEventArgs {}
impl ::core::clone::Clone for HoldingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for HoldingState {}
impl ::core::clone::Clone for HoldingState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAttachableInputObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAttachableInputObject {}
impl ::core::clone::Clone for IAttachableInputObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAttachableInputObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAttachableInputObjectFactory {}
impl ::core::clone::Clone for IAttachableInputObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrossSlidingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrossSlidingEventArgs {}
impl ::core::clone::Clone for ICrossSlidingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICrossSlidingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICrossSlidingEventArgs2 {}
impl ::core::clone::Clone for ICrossSlidingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDraggingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDraggingEventArgs {}
impl ::core::clone::Clone for IDraggingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDraggingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDraggingEventArgs2 {}
impl ::core::clone::Clone for IDraggingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEdgeGesture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEdgeGesture {}
impl ::core::clone::Clone for IEdgeGesture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEdgeGestureEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEdgeGestureEventArgs {}
impl ::core::clone::Clone for IEdgeGestureEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEdgeGestureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEdgeGestureStatics {}
impl ::core::clone::Clone for IEdgeGestureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGestureRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGestureRecognizer {}
impl ::core::clone::Clone for IGestureRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGestureRecognizer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGestureRecognizer2 {}
impl ::core::clone::Clone for IGestureRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHoldingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHoldingEventArgs {}
impl ::core::clone::Clone for IHoldingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHoldingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHoldingEventArgs2 {}
impl ::core::clone::Clone for IHoldingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputActivationListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputActivationListener {}
impl ::core::clone::Clone for IInputActivationListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputActivationListenerActivationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputActivationListenerActivationChangedEventArgs {}
impl ::core::clone::Clone for IInputActivationListenerActivationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyboardDeliveryInterceptor {}
impl ::core::clone::Clone for IKeyboardDeliveryInterceptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyboardDeliveryInterceptorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyboardDeliveryInterceptorStatics {}
impl ::core::clone::Clone for IKeyboardDeliveryInterceptorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationCompletedEventArgs {}
impl ::core::clone::Clone for IManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationCompletedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationCompletedEventArgs2 {}
impl ::core::clone::Clone for IManipulationCompletedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationInertiaStartingEventArgs {}
impl ::core::clone::Clone for IManipulationInertiaStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationInertiaStartingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationInertiaStartingEventArgs2 {}
impl ::core::clone::Clone for IManipulationInertiaStartingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationStartedEventArgs {}
impl ::core::clone::Clone for IManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationStartedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationStartedEventArgs2 {}
impl ::core::clone::Clone for IManipulationStartedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationUpdatedEventArgs {}
impl ::core::clone::Clone for IManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IManipulationUpdatedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IManipulationUpdatedEventArgs2 {}
impl ::core::clone::Clone for IManipulationUpdatedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMouseWheelParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMouseWheelParameters {}
impl ::core::clone::Clone for IMouseWheelParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerPoint {}
impl ::core::clone::Clone for IPointerPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerPointProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerPointProperties {}
impl ::core::clone::Clone for IPointerPointProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerPointProperties2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerPointProperties2 {}
impl ::core::clone::Clone for IPointerPointProperties2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerPointStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerPointStatics {}
impl ::core::clone::Clone for IPointerPointStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerPointTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerPointTransform {}
impl ::core::clone::Clone for IPointerPointTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerVisualizationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerVisualizationSettings {}
impl ::core::clone::Clone for IPointerVisualizationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerVisualizationSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerVisualizationSettingsStatics {}
impl ::core::clone::Clone for IPointerVisualizationSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialController {}
impl ::core::clone::Clone for IRadialController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialController2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialController2 {}
impl ::core::clone::Clone for IRadialController2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerButtonClickedEventArgs {}
impl ::core::clone::Clone for IRadialControllerButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerButtonClickedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerButtonClickedEventArgs2 {}
impl ::core::clone::Clone for IRadialControllerButtonClickedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerButtonHoldingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerButtonHoldingEventArgs {}
impl ::core::clone::Clone for IRadialControllerButtonHoldingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerButtonPressedEventArgs {}
impl ::core::clone::Clone for IRadialControllerButtonPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerButtonReleasedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerButtonReleasedEventArgs {}
impl ::core::clone::Clone for IRadialControllerButtonReleasedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerConfiguration {}
impl ::core::clone::Clone for IRadialControllerConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerConfiguration2 {}
impl ::core::clone::Clone for IRadialControllerConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerConfigurationStatics {}
impl ::core::clone::Clone for IRadialControllerConfigurationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerConfigurationStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerConfigurationStatics2 {}
impl ::core::clone::Clone for IRadialControllerConfigurationStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerControlAcquiredEventArgs {}
impl ::core::clone::Clone for IRadialControllerControlAcquiredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerControlAcquiredEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerControlAcquiredEventArgs2 {}
impl ::core::clone::Clone for IRadialControllerControlAcquiredEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerMenu {}
impl ::core::clone::Clone for IRadialControllerMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerMenuItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerMenuItem {}
impl ::core::clone::Clone for IRadialControllerMenuItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerMenuItemStatics {}
impl ::core::clone::Clone for IRadialControllerMenuItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerMenuItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerMenuItemStatics2 {}
impl ::core::clone::Clone for IRadialControllerMenuItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerRotationChangedEventArgs {}
impl ::core::clone::Clone for IRadialControllerRotationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerRotationChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerRotationChangedEventArgs2 {}
impl ::core::clone::Clone for IRadialControllerRotationChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContact {}
impl ::core::clone::Clone for IRadialControllerScreenContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContactContinuedEventArgs {}
impl ::core::clone::Clone for IRadialControllerScreenContactContinuedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContactContinuedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContactContinuedEventArgs2 {}
impl ::core::clone::Clone for IRadialControllerScreenContactContinuedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContactEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContactEndedEventArgs {}
impl ::core::clone::Clone for IRadialControllerScreenContactEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContactStartedEventArgs {}
impl ::core::clone::Clone for IRadialControllerScreenContactStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerScreenContactStartedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerScreenContactStartedEventArgs2 {}
impl ::core::clone::Clone for IRadialControllerScreenContactStartedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerStatics {}
impl ::core::clone::Clone for IRadialControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRightTappedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRightTappedEventArgs {}
impl ::core::clone::Clone for IRightTappedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRightTappedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRightTappedEventArgs2 {}
impl ::core::clone::Clone for IRightTappedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemButtonEventController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemButtonEventController {}
impl ::core::clone::Clone for ISystemButtonEventController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemButtonEventControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemButtonEventControllerStatics {}
impl ::core::clone::Clone for ISystemButtonEventControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemFunctionButtonEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemFunctionButtonEventArgs {}
impl ::core::clone::Clone for ISystemFunctionButtonEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemFunctionLockChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemFunctionLockChangedEventArgs {}
impl ::core::clone::Clone for ISystemFunctionLockChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemFunctionLockIndicatorChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemFunctionLockIndicatorChangedEventArgs {}
impl ::core::clone::Clone for ISystemFunctionLockIndicatorChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITappedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITappedEventArgs {}
impl ::core::clone::Clone for ITappedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITappedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITappedEventArgs2 {}
impl ::core::clone::Clone for ITappedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputActivationListener(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InputActivationListener {}
impl ::core::clone::Clone for InputActivationListener {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputActivationListenerActivationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InputActivationListenerActivationChangedEventArgs {}
impl ::core::clone::Clone for InputActivationListenerActivationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for InputActivationState {}
impl ::core::clone::Clone for InputActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyboardDeliveryInterceptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyboardDeliveryInterceptor {}
impl ::core::clone::Clone for KeyboardDeliveryInterceptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManipulationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ManipulationCompletedEventArgs {}
impl ::core::clone::Clone for ManipulationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct ManipulationDelta {
    pub Translation: super::super::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationDelta {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationDelta {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ManipulationInertiaStartingEventArgs {}
impl ::core::clone::Clone for ManipulationInertiaStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManipulationStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ManipulationStartedEventArgs {}
impl ::core::clone::Clone for ManipulationStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ManipulationUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ManipulationUpdatedEventArgs {}
impl ::core::clone::Clone for ManipulationUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct ManipulationVelocities {
    pub Linear: super::super::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for ManipulationVelocities {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for ManipulationVelocities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MouseWheelParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MouseWheelParameters {}
impl ::core::clone::Clone for MouseWheelParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerPoint {}
impl ::core::clone::Clone for PointerPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerPointProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerPointProperties {}
impl ::core::clone::Clone for PointerPointProperties {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PointerUpdateKind {}
impl ::core::clone::Clone for PointerUpdateKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerVisualizationSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerVisualizationSettings {}
impl ::core::clone::Clone for PointerVisualizationSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialController {}
impl ::core::clone::Clone for RadialController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerButtonClickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerButtonClickedEventArgs {}
impl ::core::clone::Clone for RadialControllerButtonClickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerButtonHoldingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerButtonHoldingEventArgs {}
impl ::core::clone::Clone for RadialControllerButtonHoldingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerButtonPressedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerButtonPressedEventArgs {}
impl ::core::clone::Clone for RadialControllerButtonPressedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerButtonReleasedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerButtonReleasedEventArgs {}
impl ::core::clone::Clone for RadialControllerButtonReleasedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerConfiguration {}
impl ::core::clone::Clone for RadialControllerConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerControlAcquiredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerControlAcquiredEventArgs {}
impl ::core::clone::Clone for RadialControllerControlAcquiredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerMenu {}
impl ::core::clone::Clone for RadialControllerMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerMenuItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerMenuItem {}
impl ::core::clone::Clone for RadialControllerMenuItem {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RadialControllerMenuKnownIcon {}
impl ::core::clone::Clone for RadialControllerMenuKnownIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerRotationChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerRotationChangedEventArgs {}
impl ::core::clone::Clone for RadialControllerRotationChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerScreenContact(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerScreenContact {}
impl ::core::clone::Clone for RadialControllerScreenContact {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerScreenContactContinuedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerScreenContactContinuedEventArgs {}
impl ::core::clone::Clone for RadialControllerScreenContactContinuedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerScreenContactEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerScreenContactEndedEventArgs {}
impl ::core::clone::Clone for RadialControllerScreenContactEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerScreenContactStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerScreenContactStartedEventArgs {}
impl ::core::clone::Clone for RadialControllerScreenContactStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerSystemMenuItemKind(pub i32);
impl RadialControllerSystemMenuItemKind {
    pub const Scroll: Self = Self(0i32);
    pub const Zoom: Self = Self(1i32);
    pub const UndoRedo: Self = Self(2i32);
    pub const Volume: Self = Self(3i32);
    pub const NextPreviousTrack: Self = Self(4i32);
}
impl ::core::marker::Copy for RadialControllerSystemMenuItemKind {}
impl ::core::clone::Clone for RadialControllerSystemMenuItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RightTappedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RightTappedEventArgs {}
impl ::core::clone::Clone for RightTappedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemButtonEventController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemButtonEventController {}
impl ::core::clone::Clone for SystemButtonEventController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemFunctionButtonEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemFunctionButtonEventArgs {}
impl ::core::clone::Clone for SystemFunctionButtonEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemFunctionLockChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemFunctionLockChangedEventArgs {}
impl ::core::clone::Clone for SystemFunctionLockChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemFunctionLockIndicatorChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemFunctionLockIndicatorChangedEventArgs {}
impl ::core::clone::Clone for SystemFunctionLockIndicatorChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TappedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TappedEventArgs {}
impl ::core::clone::Clone for TappedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
