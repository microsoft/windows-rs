#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AcceleratorKeyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AcceleratorKeyEventArgs {}
impl ::core::clone::Clone for AcceleratorKeyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for AppViewBackButtonVisibility {}
impl ::core::clone::Clone for AppViewBackButtonVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationProviderRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationProviderRequestedEventArgs {}
impl ::core::clone::Clone for AutomationProviderRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackRequestedEventArgs {}
impl ::core::clone::Clone for BackRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CharacterReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CharacterReceivedEventArgs {}
impl ::core::clone::Clone for CharacterReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClosestInteractiveBoundsRequestedEventArgs {}
impl ::core::clone::Clone for ClosestInteractiveBoundsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreAcceleratorKeyEventType(pub i32);
impl CoreAcceleratorKeyEventType {
    pub const Character: Self = Self(2i32);
    pub const DeadCharacter: Self = Self(3i32);
    pub const KeyDown: Self = Self(0i32);
    pub const KeyUp: Self = Self(1i32);
    pub const SystemCharacter: Self = Self(6i32);
    pub const SystemDeadCharacter: Self = Self(7i32);
    pub const SystemKeyDown: Self = Self(4i32);
    pub const SystemKeyUp: Self = Self(5i32);
    pub const UnicodeCharacter: Self = Self(8i32);
}
impl ::core::marker::Copy for CoreAcceleratorKeyEventType {}
impl ::core::clone::Clone for CoreAcceleratorKeyEventType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreAcceleratorKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreAcceleratorKeys {}
impl ::core::clone::Clone for CoreAcceleratorKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreComponentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreComponentInputSource {}
impl ::core::clone::Clone for CoreComponentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreCursor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreCursor {}
impl ::core::clone::Clone for CoreCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreCursorType(pub i32);
impl CoreCursorType {
    pub const Arrow: Self = Self(0i32);
    pub const Cross: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Hand: Self = Self(3i32);
    pub const Help: Self = Self(4i32);
    pub const IBeam: Self = Self(5i32);
    pub const SizeAll: Self = Self(6i32);
    pub const SizeNortheastSouthwest: Self = Self(7i32);
    pub const SizeNorthSouth: Self = Self(8i32);
    pub const SizeNorthwestSoutheast: Self = Self(9i32);
    pub const SizeWestEast: Self = Self(10i32);
    pub const UniversalNo: Self = Self(11i32);
    pub const UpArrow: Self = Self(12i32);
    pub const Wait: Self = Self(13i32);
    pub const Pin: Self = Self(14i32);
    pub const Person: Self = Self(15i32);
}
impl ::core::marker::Copy for CoreCursorType {}
impl ::core::clone::Clone for CoreCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDispatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreDispatcher {}
impl ::core::clone::Clone for CoreDispatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: Self = Self(-2i32);
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreDispatcherPriority {}
impl ::core::clone::Clone for CoreDispatcherPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreIndependentInputFilters(pub u32);
impl CoreIndependentInputFilters {
    pub const None: Self = Self(0u32);
    pub const MouseButton: Self = Self(1u32);
    pub const MouseWheel: Self = Self(2u32);
    pub const MouseHover: Self = Self(4u32);
    pub const PenWithBarrel: Self = Self(8u32);
    pub const PenInverted: Self = Self(16u32);
}
impl ::core::marker::Copy for CoreIndependentInputFilters {}
impl ::core::clone::Clone for CoreIndependentInputFilters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreIndependentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreIndependentInputSource {}
impl ::core::clone::Clone for CoreIndependentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreIndependentInputSourceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreIndependentInputSourceController {}
impl ::core::clone::Clone for CoreIndependentInputSourceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
impl ::core::marker::Copy for CoreInputDeviceTypes {}
impl ::core::clone::Clone for CoreInputDeviceTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CorePhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl ::core::marker::Copy for CorePhysicalKeyStatus {}
impl ::core::clone::Clone for CorePhysicalKeyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: Self = Self(0i32);
    pub const ProcessOneIfPresent: Self = Self(1i32);
    pub const ProcessUntilQuit: Self = Self(2i32);
    pub const ProcessAllIfPresent: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreProcessEventsOption {}
impl ::core::clone::Clone for CoreProcessEventsOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct CoreProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: super::super::Foundation::Point,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CoreProximityEvaluation {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CoreProximityEvaluation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: Self = Self(0i32);
    pub const Farthest: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for CoreProximityEvaluationScore {}
impl ::core::clone::Clone for CoreProximityEvaluationScore {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
impl ::core::marker::Copy for CoreVirtualKeyStates {}
impl ::core::clone::Clone for CoreVirtualKeyStates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindow {}
impl ::core::clone::Clone for CoreWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
impl ::core::marker::Copy for CoreWindowActivationMode {}
impl ::core::clone::Clone for CoreWindowActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const PointerActivated: Self = Self(2i32);
}
impl ::core::marker::Copy for CoreWindowActivationState {}
impl ::core::clone::Clone for CoreWindowActivationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindowDialog {}
impl ::core::clone::Clone for CoreWindowDialog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindowEventArgs {}
impl ::core::clone::Clone for CoreWindowEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for CoreWindowFlowDirection {}
impl ::core::clone::Clone for CoreWindowFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindowFlyout {}
impl ::core::clone::Clone for CoreWindowFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowPopupShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindowPopupShowingEventArgs {}
impl ::core::clone::Clone for CoreWindowPopupShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CoreWindowResizeManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreWindowResizeManager {}
impl ::core::clone::Clone for CoreWindowResizeManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DispatchedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DispatchedHandler {}
impl ::core::clone::Clone for DispatchedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcceleratorKeyEventArgs {}
impl ::core::clone::Clone for IAcceleratorKeyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAcceleratorKeyEventArgs2 {}
impl ::core::clone::Clone for IAcceleratorKeyEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationProviderRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationProviderRequestedEventArgs {}
impl ::core::clone::Clone for IAutomationProviderRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackRequestedEventArgs {}
impl ::core::clone::Clone for IBackRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICharacterReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICharacterReceivedEventArgs {}
impl ::core::clone::Clone for ICharacterReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClosestInteractiveBoundsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClosestInteractiveBoundsRequestedEventArgs {}
impl ::core::clone::Clone for IClosestInteractiveBoundsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAcceleratorKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAcceleratorKeys {}
impl ::core::clone::Clone for ICoreAcceleratorKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreClosestInteractiveBoundsRequested(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreClosestInteractiveBoundsRequested {}
impl ::core::clone::Clone for ICoreClosestInteractiveBoundsRequested {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreComponentFocusable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreComponentFocusable {}
impl ::core::clone::Clone for ICoreComponentFocusable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreCursor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreCursor {}
impl ::core::clone::Clone for ICoreCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreCursorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreCursorFactory {}
impl ::core::clone::Clone for ICoreCursorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDispatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDispatcher {}
impl ::core::clone::Clone for ICoreDispatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDispatcher2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDispatcher2 {}
impl ::core::clone::Clone for ICoreDispatcher2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreDispatcherWithTaskPriority(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreDispatcherWithTaskPriority {}
impl ::core::clone::Clone for ICoreDispatcherWithTaskPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreIndependentInputSourceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreIndependentInputSourceController {}
impl ::core::clone::Clone for ICoreIndependentInputSourceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreIndependentInputSourceControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreIndependentInputSourceControllerStatics {}
impl ::core::clone::Clone for ICoreIndependentInputSourceControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreInputSourceBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreInputSourceBase {}
impl ::core::clone::Clone for ICoreInputSourceBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreKeyboardInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreKeyboardInputSource {}
impl ::core::clone::Clone for ICoreKeyboardInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreKeyboardInputSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreKeyboardInputSource2 {}
impl ::core::clone::Clone for ICoreKeyboardInputSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICorePointerInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICorePointerInputSource {}
impl ::core::clone::Clone for ICorePointerInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICorePointerInputSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICorePointerInputSource2 {}
impl ::core::clone::Clone for ICorePointerInputSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICorePointerRedirector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICorePointerRedirector {}
impl ::core::clone::Clone for ICorePointerRedirector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreTouchHitTesting(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreTouchHitTesting {}
impl ::core::clone::Clone for ICoreTouchHitTesting {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindow {}
impl ::core::clone::Clone for ICoreWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindow2 {}
impl ::core::clone::Clone for ICoreWindow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindow3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindow3 {}
impl ::core::clone::Clone for ICoreWindow3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindow4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindow4 {}
impl ::core::clone::Clone for ICoreWindow4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindow5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindow5 {}
impl ::core::clone::Clone for ICoreWindow5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowDialog {}
impl ::core::clone::Clone for ICoreWindowDialog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowDialogFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowDialogFactory {}
impl ::core::clone::Clone for ICoreWindowDialogFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowEventArgs {}
impl ::core::clone::Clone for ICoreWindowEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowFlyout {}
impl ::core::clone::Clone for ICoreWindowFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowFlyoutFactory {}
impl ::core::clone::Clone for ICoreWindowFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowPopupShowingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowPopupShowingEventArgs {}
impl ::core::clone::Clone for ICoreWindowPopupShowingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowResizeManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowResizeManager {}
impl ::core::clone::Clone for ICoreWindowResizeManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowResizeManagerLayoutCapability(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowResizeManagerLayoutCapability {}
impl ::core::clone::Clone for ICoreWindowResizeManagerLayoutCapability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowResizeManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowResizeManagerStatics {}
impl ::core::clone::Clone for ICoreWindowResizeManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowStatic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowStatic {}
impl ::core::clone::Clone for ICoreWindowStatic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreWindowWithContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreWindowWithContext {}
impl ::core::clone::Clone for ICoreWindowWithContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdleDispatchedHandlerArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdleDispatchedHandlerArgs {}
impl ::core::clone::Clone for IIdleDispatchedHandlerArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInitializeWithCoreWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInitializeWithCoreWindow {}
impl ::core::clone::Clone for IInitializeWithCoreWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputEnabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputEnabledEventArgs {}
impl ::core::clone::Clone for IInputEnabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyEventArgs {}
impl ::core::clone::Clone for IKeyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKeyEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKeyEventArgs2 {}
impl ::core::clone::Clone for IKeyEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPointerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPointerEventArgs {}
impl ::core::clone::Clone for IPointerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationManager {}
impl ::core::clone::Clone for ISystemNavigationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationManager2 {}
impl ::core::clone::Clone for ISystemNavigationManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationManagerStatics {}
impl ::core::clone::Clone for ISystemNavigationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITouchHitTestingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITouchHitTestingEventArgs {}
impl ::core::clone::Clone for ITouchHitTestingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisibilityChangedEventArgs {}
impl ::core::clone::Clone for IVisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowActivatedEventArgs {}
impl ::core::clone::Clone for IWindowActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowSizeChangedEventArgs {}
impl ::core::clone::Clone for IWindowSizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IdleDispatchedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IdleDispatchedHandler {}
impl ::core::clone::Clone for IdleDispatchedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IdleDispatchedHandlerArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IdleDispatchedHandlerArgs {}
impl ::core::clone::Clone for IdleDispatchedHandlerArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputEnabledEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InputEnabledEventArgs {}
impl ::core::clone::Clone for InputEnabledEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KeyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for KeyEventArgs {}
impl ::core::clone::Clone for KeyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PointerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PointerEventArgs {}
impl ::core::clone::Clone for PointerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemNavigationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemNavigationManager {}
impl ::core::clone::Clone for SystemNavigationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TouchHitTestingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TouchHitTestingEventArgs {}
impl ::core::clone::Clone for TouchHitTestingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisibilityChangedEventArgs {}
impl ::core::clone::Clone for VisibilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowActivatedEventArgs {}
impl ::core::clone::Clone for WindowActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowSizeChangedEventArgs {}
impl ::core::clone::Clone for WindowSizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
