#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccessKeyDisplayDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccessKeyDisplayRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccessKeyInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AccessKeyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CanExecuteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CharacterReceivedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleTappedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DoubleTappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExecuteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FindNextElementOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FocusInputDeviceKind(i32);
#[repr(transparent)]
pub struct FocusManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusManagerGotFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusManagerLostFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusMovementResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FocusNavigationDirection(i32);
#[repr(transparent)]
pub struct GettingFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HoldingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HoldingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyDisplayDismissedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyDisplayRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessKeyManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanExecuteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICharacterReceivedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDoubleTappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecuteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFindNextElementOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerGotFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerLostFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusManagerStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusMovementResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGettingFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGettingFocusEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGettingFocusEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHoldingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInertiaExpansionBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInertiaRotationBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInertiaTranslationBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputScope(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputScopeName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputScopeNameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyRoutedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyRoutedEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardAcceleratorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardAcceleratorInvokedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyboardAcceleratorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILosingFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILosingFocusEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILosingFocusEventArgs3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationCompletedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationDeltaRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationInertiaStartingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationPivot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationPivotFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationStartedRoutedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationStartingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INoFocusCandidateFoundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPointerRoutedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessKeyboardAcceleratorEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRightTappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardUICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardUICommand2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardUICommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardUICommandStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUICommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXamlUICommandStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InertiaExpansionBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InertiaRotationBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InertiaTranslationBehavior(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputScope(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputScopeName(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct InputScopeNameValue(i32);
#[repr(transparent)]
pub struct KeyEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct KeyTipPlacementMode(i32);
#[repr(transparent)]
pub struct KeyboardAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct KeyboardAcceleratorPlacementMode(i32);
#[repr(C)]
pub struct KeyboardNavigationMode(i32);
#[repr(transparent)]
pub struct LosingFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationCompletedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationCompletedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationDeltaEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationDeltaRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationInertiaStartingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationInertiaStartingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ManipulationModes(i32);
#[repr(transparent)]
pub struct ManipulationPivot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationStartedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationStartedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationStartingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManipulationStartingRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NoFocusCandidateFoundEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Pointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PointerRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessKeyboardAcceleratorEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RightTappedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RightTappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StandardUICommand(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct StandardUICommandKind(i32);
#[repr(transparent)]
pub struct TappedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct XYFocusKeyboardNavigationMode(i32);
#[repr(C)]
pub struct XYFocusNavigationStrategy(i32);
#[repr(C)]
pub struct XYFocusNavigationStrategyOverride(i32);
#[repr(transparent)]
pub struct XamlUICommand(pub *mut ::core::ffi::c_void);
