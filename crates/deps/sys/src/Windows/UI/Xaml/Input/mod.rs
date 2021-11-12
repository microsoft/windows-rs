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
    pub const None: Self = Self(0i32);
    pub const Mouse: Self = Self(1i32);
    pub const Touch: Self = Self(2i32);
    pub const Pen: Self = Self(3i32);
    pub const Keyboard: Self = Self(4i32);
    pub const GameController: Self = Self(5i32);
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
    pub const Next: Self = Self(0i32);
    pub const Previous: Self = Self(1i32);
    pub const Up: Self = Self(2i32);
    pub const Down: Self = Self(3i32);
    pub const Left: Self = Self(4i32);
    pub const Right: Self = Self(5i32);
    pub const None: Self = Self(6i32);
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
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const EmailSmtpAddress: Self = Self(5i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const DateMonthNumber: Self = Self(23i32);
    pub const DateDayNumber: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const Digits: Self = Self(28i32);
    pub const Number: Self = Self(29i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinutesOrSeconds: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const Hiragana: Self = Self(44i32);
    pub const KatakanaHalfWidth: Self = Self(45i32);
    pub const KatakanaFullWidth: Self = Self(46i32);
    pub const Hanja: Self = Self(47i32);
    pub const HangulHalfWidth: Self = Self(48i32);
    pub const HangulFullWidth: Self = Self(49i32);
    pub const Search: Self = Self(50i32);
    pub const Formula: Self = Self(51i32);
    pub const SearchIncremental: Self = Self(52i32);
    pub const ChineseHalfWidth: Self = Self(53i32);
    pub const ChineseFullWidth: Self = Self(54i32);
    pub const NativeScript: Self = Self(55i32);
    pub const Text: Self = Self(57i32);
    pub const Chat: Self = Self(58i32);
    pub const NameOrPhoneNumber: Self = Self(59i32);
    pub const EmailNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const NumericPassword: Self = Self(63i32);
    pub const NumericPin: Self = Self(64i32);
    pub const AlphanumericPin: Self = Self(65i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
}
#[repr(transparent)]
pub struct KeyEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const Center: Self = Self(5i32);
    pub const Hidden: Self = Self(6i32);
}
#[repr(transparent)]
pub struct KeyboardAccelerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardAcceleratorInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
#[repr(transparent)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0i32);
    pub const Cycle: Self = Self(1i32);
    pub const Once: Self = Self(2i32);
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
    pub const None: Self = Self(0u32);
    pub const TranslateX: Self = Self(1u32);
    pub const TranslateY: Self = Self(2u32);
    pub const TranslateRailsX: Self = Self(4u32);
    pub const TranslateRailsY: Self = Self(8u32);
    pub const Rotate: Self = Self(16u32);
    pub const Scale: Self = Self(32u32);
    pub const TranslateInertia: Self = Self(64u32);
    pub const RotateInertia: Self = Self(128u32);
    pub const ScaleInertia: Self = Self(256u32);
    pub const All: Self = Self(65535u32);
    pub const System: Self = Self(65536u32);
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
    pub const None: Self = Self(0i32);
    pub const Cut: Self = Self(1i32);
    pub const Copy: Self = Self(2i32);
    pub const Paste: Self = Self(3i32);
    pub const SelectAll: Self = Self(4i32);
    pub const Delete: Self = Self(5i32);
    pub const Share: Self = Self(6i32);
    pub const Save: Self = Self(7i32);
    pub const Open: Self = Self(8i32);
    pub const Close: Self = Self(9i32);
    pub const Pause: Self = Self(10i32);
    pub const Play: Self = Self(11i32);
    pub const Stop: Self = Self(12i32);
    pub const Forward: Self = Self(13i32);
    pub const Backward: Self = Self(14i32);
    pub const Undo: Self = Self(15i32);
    pub const Redo: Self = Self(16i32);
}
#[repr(transparent)]
pub struct TappedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TappedRoutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0i32);
    pub const Projection: Self = Self(1i32);
    pub const NavigationDirectionDistance: Self = Self(2i32);
    pub const RectilinearDistance: Self = Self(3i32);
}
#[repr(transparent)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Projection: Self = Self(2i32);
    pub const NavigationDirectionDistance: Self = Self(3i32);
    pub const RectilinearDistance: Self = Self(4i32);
}
#[repr(transparent)]
pub struct XamlUICommand(pub *mut ::core::ffi::c_void);
