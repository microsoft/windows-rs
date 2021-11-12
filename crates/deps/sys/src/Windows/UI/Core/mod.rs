#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AcceleratorKeyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppViewBackButtonVisibility(pub i32);
impl AppViewBackButtonVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AutomationProviderRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CharacterReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct CoreAcceleratorKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreComponentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreCursor(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct CoreDispatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreDispatcherPriority(pub i32);
impl CoreDispatcherPriority {
    pub const Idle: Self = Self(-2i32);
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
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
#[repr(transparent)]
pub struct CoreIndependentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreIndependentInputSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreInputDeviceTypes(pub u32);
impl CoreInputDeviceTypes {
    pub const None: Self = Self(0u32);
    pub const Touch: Self = Self(1u32);
    pub const Pen: Self = Self(2u32);
    pub const Mouse: Self = Self(4u32);
}
#[repr(C)]
pub struct CorePhysicalKeyStatus(i32);
#[repr(transparent)]
pub struct CoreProcessEventsOption(pub i32);
impl CoreProcessEventsOption {
    pub const ProcessOneAndAllPending: Self = Self(0i32);
    pub const ProcessOneIfPresent: Self = Self(1i32);
    pub const ProcessUntilQuit: Self = Self(2i32);
    pub const ProcessAllIfPresent: Self = Self(3i32);
}
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct CoreProximityEvaluation(i32);
#[repr(transparent)]
pub struct CoreProximityEvaluationScore(pub i32);
impl CoreProximityEvaluationScore {
    pub const Closest: Self = Self(0i32);
    pub const Farthest: Self = Self(2147483647i32);
}
#[repr(transparent)]
pub struct CoreVirtualKeyStates(pub u32);
impl CoreVirtualKeyStates {
    pub const None: Self = Self(0u32);
    pub const Down: Self = Self(1u32);
    pub const Locked: Self = Self(2u32);
}
#[repr(transparent)]
pub struct CoreWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWindowActivationMode(pub i32);
impl CoreWindowActivationMode {
    pub const None: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const ActivatedNotForeground: Self = Self(2i32);
    pub const ActivatedInForeground: Self = Self(3i32);
}
#[repr(transparent)]
pub struct CoreWindowActivationState(pub i32);
impl CoreWindowActivationState {
    pub const CodeActivated: Self = Self(0i32);
    pub const Deactivated: Self = Self(1i32);
    pub const PointerActivated: Self = Self(2i32);
}
#[repr(transparent)]
pub struct CoreWindowDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWindowEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWindowFlowDirection(pub i32);
impl CoreWindowFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CoreWindowFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWindowPopupShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreWindowResizeManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DispatchedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAcceleratorKeyEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationProviderRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICharacterReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClosestInteractiveBoundsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAcceleratorKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreClosestInteractiveBoundsRequested(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreComponentFocusable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreCursor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreCursorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDispatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDispatcher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreDispatcherWithTaskPriority(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreIndependentInputSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreIndependentInputSourceControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreInputSourceBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreKeyboardInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreKeyboardInputSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorePointerInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorePointerInputSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICorePointerRedirector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTouchHitTesting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindow3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindow4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindow5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowDialogFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowPopupShowingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowResizeManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowResizeManagerLayoutCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowResizeManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreWindowWithContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdleDispatchedHandlerArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeWithCoreWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputEnabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITouchHitTestingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IdleDispatchedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IdleDispatchedHandlerArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputEnabledEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemNavigationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TouchHitTestingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisibilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
