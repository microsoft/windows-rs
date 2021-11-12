#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreTextCompositionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextCompositionSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextEditContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextFormatUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(0i32);
    pub const CompositionUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(1i32);
    pub const CompositionConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(2i32);
    pub const CompositionTargetUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(3i32);
    pub const CompositionTargetConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(4i32);
}
#[repr(transparent)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(0i32);
    pub const Failed: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(1i32);
}
#[repr(transparent)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(0i32);
    pub const Manual: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(1i32);
}
#[repr(transparent)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: CoreTextInputScope = CoreTextInputScope(0i32);
    pub const Url: CoreTextInputScope = CoreTextInputScope(1i32);
    pub const FilePath: CoreTextInputScope = CoreTextInputScope(2i32);
    pub const FileName: CoreTextInputScope = CoreTextInputScope(3i32);
    pub const EmailUserName: CoreTextInputScope = CoreTextInputScope(4i32);
    pub const EmailAddress: CoreTextInputScope = CoreTextInputScope(5i32);
    pub const UserName: CoreTextInputScope = CoreTextInputScope(6i32);
    pub const PersonalFullName: CoreTextInputScope = CoreTextInputScope(7i32);
    pub const PersonalNamePrefix: CoreTextInputScope = CoreTextInputScope(8i32);
    pub const PersonalGivenName: CoreTextInputScope = CoreTextInputScope(9i32);
    pub const PersonalMiddleName: CoreTextInputScope = CoreTextInputScope(10i32);
    pub const PersonalSurname: CoreTextInputScope = CoreTextInputScope(11i32);
    pub const PersonalNameSuffix: CoreTextInputScope = CoreTextInputScope(12i32);
    pub const Address: CoreTextInputScope = CoreTextInputScope(13i32);
    pub const AddressPostalCode: CoreTextInputScope = CoreTextInputScope(14i32);
    pub const AddressStreet: CoreTextInputScope = CoreTextInputScope(15i32);
    pub const AddressStateOrProvince: CoreTextInputScope = CoreTextInputScope(16i32);
    pub const AddressCity: CoreTextInputScope = CoreTextInputScope(17i32);
    pub const AddressCountryName: CoreTextInputScope = CoreTextInputScope(18i32);
    pub const AddressCountryShortName: CoreTextInputScope = CoreTextInputScope(19i32);
    pub const CurrencyAmountAndSymbol: CoreTextInputScope = CoreTextInputScope(20i32);
    pub const CurrencyAmount: CoreTextInputScope = CoreTextInputScope(21i32);
    pub const Date: CoreTextInputScope = CoreTextInputScope(22i32);
    pub const DateMonth: CoreTextInputScope = CoreTextInputScope(23i32);
    pub const DateDay: CoreTextInputScope = CoreTextInputScope(24i32);
    pub const DateYear: CoreTextInputScope = CoreTextInputScope(25i32);
    pub const DateMonthName: CoreTextInputScope = CoreTextInputScope(26i32);
    pub const DateDayName: CoreTextInputScope = CoreTextInputScope(27i32);
    pub const Number: CoreTextInputScope = CoreTextInputScope(29i32);
    pub const SingleCharacter: CoreTextInputScope = CoreTextInputScope(30i32);
    pub const Password: CoreTextInputScope = CoreTextInputScope(31i32);
    pub const TelephoneNumber: CoreTextInputScope = CoreTextInputScope(32i32);
    pub const TelephoneCountryCode: CoreTextInputScope = CoreTextInputScope(33i32);
    pub const TelephoneAreaCode: CoreTextInputScope = CoreTextInputScope(34i32);
    pub const TelephoneLocalNumber: CoreTextInputScope = CoreTextInputScope(35i32);
    pub const Time: CoreTextInputScope = CoreTextInputScope(36i32);
    pub const TimeHour: CoreTextInputScope = CoreTextInputScope(37i32);
    pub const TimeMinuteOrSecond: CoreTextInputScope = CoreTextInputScope(38i32);
    pub const NumberFullWidth: CoreTextInputScope = CoreTextInputScope(39i32);
    pub const AlphanumericHalfWidth: CoreTextInputScope = CoreTextInputScope(40i32);
    pub const AlphanumericFullWidth: CoreTextInputScope = CoreTextInputScope(41i32);
    pub const CurrencyChinese: CoreTextInputScope = CoreTextInputScope(42i32);
    pub const Bopomofo: CoreTextInputScope = CoreTextInputScope(43i32);
    pub const Hiragana: CoreTextInputScope = CoreTextInputScope(44i32);
    pub const KatakanaHalfWidth: CoreTextInputScope = CoreTextInputScope(45i32);
    pub const KatakanaFullWidth: CoreTextInputScope = CoreTextInputScope(46i32);
    pub const Hanja: CoreTextInputScope = CoreTextInputScope(47i32);
    pub const HangulHalfWidth: CoreTextInputScope = CoreTextInputScope(48i32);
    pub const HangulFullWidth: CoreTextInputScope = CoreTextInputScope(49i32);
    pub const Search: CoreTextInputScope = CoreTextInputScope(50i32);
    pub const Formula: CoreTextInputScope = CoreTextInputScope(51i32);
    pub const SearchIncremental: CoreTextInputScope = CoreTextInputScope(52i32);
    pub const ChineseHalfWidth: CoreTextInputScope = CoreTextInputScope(53i32);
    pub const ChineseFullWidth: CoreTextInputScope = CoreTextInputScope(54i32);
    pub const NativeScript: CoreTextInputScope = CoreTextInputScope(55i32);
    pub const Text: CoreTextInputScope = CoreTextInputScope(57i32);
    pub const Chat: CoreTextInputScope = CoreTextInputScope(58i32);
    pub const NameOrPhoneNumber: CoreTextInputScope = CoreTextInputScope(59i32);
    pub const EmailUserNameOrAddress: CoreTextInputScope = CoreTextInputScope(60i32);
    pub const Private: CoreTextInputScope = CoreTextInputScope(61i32);
    pub const Maps: CoreTextInputScope = CoreTextInputScope(62i32);
    pub const PasswordNumeric: CoreTextInputScope = CoreTextInputScope(63i32);
    pub const FormulaNumber: CoreTextInputScope = CoreTextInputScope(67i32);
    pub const ChatWithoutEmoji: CoreTextInputScope = CoreTextInputScope(68i32);
    pub const Digits: CoreTextInputScope = CoreTextInputScope(28i32);
    pub const PinNumeric: CoreTextInputScope = CoreTextInputScope(64i32);
    pub const PinAlphanumeric: CoreTextInputScope = CoreTextInputScope(65i32);
}
#[repr(transparent)]
pub struct CoreTextLayoutBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextLayoutRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextLayoutRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CoreTextRange(i32);
#[repr(transparent)]
pub struct CoreTextSelectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextSelectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(0i32);
    pub const Failed: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(1i32);
}
#[repr(transparent)]
pub struct CoreTextServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(0i32);
    pub const Failed: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(1i32);
}
#[repr(transparent)]
pub struct ICoreTextCompositionCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextCompositionSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextEditContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextEditContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextFormatUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutBounds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextLayoutRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextSelectionUpdatingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextServicesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreTextTextUpdatingEventArgs(pub *mut ::core::ffi::c_void);
