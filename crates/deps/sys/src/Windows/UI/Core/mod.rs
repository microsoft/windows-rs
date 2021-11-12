#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Core_AnimationMetrics")]
pub mod AnimationMetrics;
#[cfg(feature = "UI_Core_Preview")]
pub mod Preview;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AcceleratorKeyEventArgs(pub *mut ::core::ffi::c_void);
pub struct AppViewBackButtonVisibility(i32);
#[repr(transparent)]
pub struct AutomationProviderRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CharacterReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClosestInteractiveBoundsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct CoreAcceleratorKeyEventType(i32);
#[repr(transparent)]
pub struct CoreAcceleratorKeys(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreComponentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreCursor(pub *mut ::core::ffi::c_void);
pub struct CoreCursorType(i32);
#[repr(transparent)]
pub struct CoreDispatcher(pub *mut ::core::ffi::c_void);
pub struct CoreDispatcherPriority(i32);
pub struct CoreIndependentInputFilters(i32);
#[repr(transparent)]
pub struct CoreIndependentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreIndependentInputSourceController(pub *mut ::core::ffi::c_void);
pub struct CoreInputDeviceTypes(i32);
pub struct CorePhysicalKeyStatus(i32);
pub struct CoreProcessEventsOption(i32);
#[cfg(feature = "Foundation")]
pub struct CoreProximityEvaluation(i32);
pub struct CoreProximityEvaluationScore(i32);
pub struct CoreVirtualKeyStates(i32);
#[repr(transparent)]
pub struct CoreWindow(pub *mut ::core::ffi::c_void);
pub struct CoreWindowActivationMode(i32);
pub struct CoreWindowActivationState(i32);
#[repr(transparent)]
pub struct CoreWindowDialog(pub *mut ::core::ffi::c_void);
pub struct CoreWindowDialogsContract(i32);
#[repr(transparent)]
pub struct CoreWindowEventArgs(pub *mut ::core::ffi::c_void);
pub struct CoreWindowFlowDirection(i32);
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
