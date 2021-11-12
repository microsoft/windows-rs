#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: FocusInputDeviceKind = FocusInputDeviceKind(0i32);
    pub const Mouse: FocusInputDeviceKind = FocusInputDeviceKind(1i32);
    pub const Touch: FocusInputDeviceKind = FocusInputDeviceKind(2i32);
    pub const Pen: FocusInputDeviceKind = FocusInputDeviceKind(3i32);
    pub const Keyboard: FocusInputDeviceKind = FocusInputDeviceKind(4i32);
    pub const GameController: FocusInputDeviceKind = FocusInputDeviceKind(5i32);
}
#[repr(transparent)]
pub struct FocusManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusManagerGotFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusManagerLostFocusEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusMovementResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: FocusNavigationDirection = FocusNavigationDirection(0i32);
    pub const Previous: FocusNavigationDirection = FocusNavigationDirection(1i32);
    pub const Up: FocusNavigationDirection = FocusNavigationDirection(2i32);
    pub const Down: FocusNavigationDirection = FocusNavigationDirection(3i32);
    pub const Left: FocusNavigationDirection = FocusNavigationDirection(4i32);
    pub const Right: FocusNavigationDirection = FocusNavigationDirection(5i32);
    pub const None: FocusNavigationDirection = FocusNavigationDirection(6i32);
}
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
#[repr(transparent)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: InputScopeNameValue = InputScopeNameValue(0i32);
    pub const Url: InputScopeNameValue = InputScopeNameValue(1i32);
    pub const EmailSmtpAddress: InputScopeNameValue = InputScopeNameValue(5i32);
    pub const PersonalFullName: InputScopeNameValue = InputScopeNameValue(7i32);
    pub const CurrencyAmountAndSymbol: InputScopeNameValue = InputScopeNameValue(20i32);
    pub const CurrencyAmount: InputScopeNameValue = InputScopeNameValue(21i32);
    pub const DateMonthNumber: InputScopeNameValue = InputScopeNameValue(23i32);
    pub const DateDayNumber: InputScopeNameValue = InputScopeNameValue(24i32);
    pub const DateYear: InputScopeNameValue = InputScopeNameValue(25i32);
    pub const Digits: InputScopeNameValue = InputScopeNameValue(28i32);
    pub const Number: InputScopeNameValue = InputScopeNameValue(29i32);
    pub const Password: InputScopeNameValue = InputScopeNameValue(31i32);
    pub const TelephoneNumber: InputScopeNameValue = InputScopeNameValue(32i32);
    pub const TelephoneCountryCode: InputScopeNameValue = InputScopeNameValue(33i32);
    pub const TelephoneAreaCode: InputScopeNameValue = InputScopeNameValue(34i32);
    pub const TelephoneLocalNumber: InputScopeNameValue = InputScopeNameValue(35i32);
    pub const TimeHour: InputScopeNameValue = InputScopeNameValue(37i32);
    pub const TimeMinutesOrSeconds: InputScopeNameValue = InputScopeNameValue(38i32);
    pub const NumberFullWidth: InputScopeNameValue = InputScopeNameValue(39i32);
    pub const AlphanumericHalfWidth: InputScopeNameValue = InputScopeNameValue(40i32);
    pub const AlphanumericFullWidth: InputScopeNameValue = InputScopeNameValue(41i32);
    pub const Hiragana: InputScopeNameValue = InputScopeNameValue(44i32);
    pub const KatakanaHalfWidth: InputScopeNameValue = InputScopeNameValue(45i32);
    pub const KatakanaFullWidth: InputScopeNameValue = InputScopeNameValue(46i32);
    pub const Hanja: InputScopeNameValue = InputScopeNameValue(47i32);
    pub const HangulHalfWidth: InputScopeNameValue = InputScopeNameValue(48i32);
    pub const HangulFullWidth: InputScopeNameValue = InputScopeNameValue(49i32);
    pub const Search: InputScopeNameValue = InputScopeNameValue(50i32);
    pub const Formula: InputScopeNameValue = InputScopeNameValue(51i32);
    pub const SearchIncremental: InputScopeNameValue = InputScopeNameValue(52i32);
    pub const ChineseHalfWidth: InputScopeNameValue = InputScopeNameValue(53i32);
    pub const ChineseFullWidth: InputScopeNameValue = InputScopeNameValue(54i32);
    pub const NativeScript: InputScopeNameValue = InputScopeNameValue(55i32);
    pub const Text: InputScopeNameValue = InputScopeNameValue(57i32);
    pub const Chat: InputScopeNameValue = InputScopeNameValue(58i32);
    pub const NameOrPhoneNumber: InputScopeNameValue = InputScopeNameValue(59i32);
    pub const EmailNameOrAddress: InputScopeNameValue = InputScopeNameValue(60i32);
    pub const Private: InputScopeNameValue = InputScopeNameValue(61i32);
    pub const Maps: InputScopeNameValue = InputScopeNameValue(62i32);
    pub const NumericPassword: InputScopeNameValue = InputScopeNameValue(63i32);
    pub const NumericPin: InputScopeNameValue = InputScopeNameValue(64i32);
    pub const AlphanumericPin: InputScopeNameValue = InputScopeNameValue(65i32);
    pub const FormulaNumber: InputScopeNameValue = InputScopeNameValue(67i32);
    pub const ChatWithoutEmoji: InputScopeNameValue = InputScopeNameValue(68i32);
}
#[repr(transparent)]
pub struct KeyEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: KeyTipPlacementMode = KeyTipPlacementMode(0i32);
    pub const Bottom: KeyTipPlacementMode = KeyTipPlacementMode(1i32);
    pub const Top: KeyTipPlacementMode = KeyTipPlacementMode(2i32);
    pub const Left: KeyTipPlacementMode = KeyTipPlacementMode(3i32);
    pub const Right: KeyTipPlacementMode = KeyTipPlacementMode(4i32);
    pub const Center: KeyTipPlacementMode = KeyTipPlacementMode(5i32);
    pub const Hidden: KeyTipPlacementMode = KeyTipPlacementMode(6i32);
}
#[repr(transparent)]
pub struct KeyboardAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(0i32);
    pub const Hidden: KeyboardAcceleratorPlacementMode = KeyboardAcceleratorPlacementMode(1i32);
}
#[repr(transparent)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: KeyboardNavigationMode = KeyboardNavigationMode(0i32);
    pub const Cycle: KeyboardNavigationMode = KeyboardNavigationMode(1i32);
    pub const Once: KeyboardNavigationMode = KeyboardNavigationMode(2i32);
}
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
#[repr(transparent)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: ManipulationModes = ManipulationModes(0u32);
    pub const TranslateX: ManipulationModes = ManipulationModes(1u32);
    pub const TranslateY: ManipulationModes = ManipulationModes(2u32);
    pub const TranslateRailsX: ManipulationModes = ManipulationModes(4u32);
    pub const TranslateRailsY: ManipulationModes = ManipulationModes(8u32);
    pub const Rotate: ManipulationModes = ManipulationModes(16u32);
    pub const Scale: ManipulationModes = ManipulationModes(32u32);
    pub const TranslateInertia: ManipulationModes = ManipulationModes(64u32);
    pub const RotateInertia: ManipulationModes = ManipulationModes(128u32);
    pub const ScaleInertia: ManipulationModes = ManipulationModes(256u32);
    pub const All: ManipulationModes = ManipulationModes(65535u32);
    pub const System: ManipulationModes = ManipulationModes(65536u32);
}
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
#[repr(transparent)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: StandardUICommandKind = StandardUICommandKind(0i32);
    pub const Cut: StandardUICommandKind = StandardUICommandKind(1i32);
    pub const Copy: StandardUICommandKind = StandardUICommandKind(2i32);
    pub const Paste: StandardUICommandKind = StandardUICommandKind(3i32);
    pub const SelectAll: StandardUICommandKind = StandardUICommandKind(4i32);
    pub const Delete: StandardUICommandKind = StandardUICommandKind(5i32);
    pub const Share: StandardUICommandKind = StandardUICommandKind(6i32);
    pub const Save: StandardUICommandKind = StandardUICommandKind(7i32);
    pub const Open: StandardUICommandKind = StandardUICommandKind(8i32);
    pub const Close: StandardUICommandKind = StandardUICommandKind(9i32);
    pub const Pause: StandardUICommandKind = StandardUICommandKind(10i32);
    pub const Play: StandardUICommandKind = StandardUICommandKind(11i32);
    pub const Stop: StandardUICommandKind = StandardUICommandKind(12i32);
    pub const Forward: StandardUICommandKind = StandardUICommandKind(13i32);
    pub const Backward: StandardUICommandKind = StandardUICommandKind(14i32);
    pub const Undo: StandardUICommandKind = StandardUICommandKind(15i32);
    pub const Redo: StandardUICommandKind = StandardUICommandKind(16i32);
}
#[repr(transparent)]
pub struct TappedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(0i32);
    pub const Enabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(1i32);
    pub const Disabled: XYFocusKeyboardNavigationMode = XYFocusKeyboardNavigationMode(2i32);
}
#[repr(transparent)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: XYFocusNavigationStrategy = XYFocusNavigationStrategy(0i32);
    pub const Projection: XYFocusNavigationStrategy = XYFocusNavigationStrategy(1i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategy = XYFocusNavigationStrategy(2i32);
    pub const RectilinearDistance: XYFocusNavigationStrategy = XYFocusNavigationStrategy(3i32);
}
#[repr(transparent)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(0i32);
    pub const Auto: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(1i32);
    pub const Projection: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(2i32);
    pub const NavigationDirectionDistance: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(3i32);
    pub const RectilinearDistance: XYFocusNavigationStrategyOverride = XYFocusNavigationStrategyOverride(4i32);
}
#[repr(transparent)]
pub struct XamlUICommand(pub *mut ::core::ffi::c_void);
