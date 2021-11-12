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
    pub const None: Self = Self(0i32);
    pub const CompositionUnconverted: Self = Self(1i32);
    pub const CompositionConverted: Self = Self(2i32);
    pub const CompositionTargetUnconverted: Self = Self(3i32);
    pub const CompositionTargetConverted: Self = Self(4i32);
}
#[repr(transparent)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
#[repr(transparent)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: Self = Self(0i32);
    pub const Url: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
    pub const FileName: Self = Self(3i32);
    pub const EmailUserName: Self = Self(4i32);
    pub const EmailAddress: Self = Self(5i32);
    pub const UserName: Self = Self(6i32);
    pub const PersonalFullName: Self = Self(7i32);
    pub const PersonalNamePrefix: Self = Self(8i32);
    pub const PersonalGivenName: Self = Self(9i32);
    pub const PersonalMiddleName: Self = Self(10i32);
    pub const PersonalSurname: Self = Self(11i32);
    pub const PersonalNameSuffix: Self = Self(12i32);
    pub const Address: Self = Self(13i32);
    pub const AddressPostalCode: Self = Self(14i32);
    pub const AddressStreet: Self = Self(15i32);
    pub const AddressStateOrProvince: Self = Self(16i32);
    pub const AddressCity: Self = Self(17i32);
    pub const AddressCountryName: Self = Self(18i32);
    pub const AddressCountryShortName: Self = Self(19i32);
    pub const CurrencyAmountAndSymbol: Self = Self(20i32);
    pub const CurrencyAmount: Self = Self(21i32);
    pub const Date: Self = Self(22i32);
    pub const DateMonth: Self = Self(23i32);
    pub const DateDay: Self = Self(24i32);
    pub const DateYear: Self = Self(25i32);
    pub const DateMonthName: Self = Self(26i32);
    pub const DateDayName: Self = Self(27i32);
    pub const Number: Self = Self(29i32);
    pub const SingleCharacter: Self = Self(30i32);
    pub const Password: Self = Self(31i32);
    pub const TelephoneNumber: Self = Self(32i32);
    pub const TelephoneCountryCode: Self = Self(33i32);
    pub const TelephoneAreaCode: Self = Self(34i32);
    pub const TelephoneLocalNumber: Self = Self(35i32);
    pub const Time: Self = Self(36i32);
    pub const TimeHour: Self = Self(37i32);
    pub const TimeMinuteOrSecond: Self = Self(38i32);
    pub const NumberFullWidth: Self = Self(39i32);
    pub const AlphanumericHalfWidth: Self = Self(40i32);
    pub const AlphanumericFullWidth: Self = Self(41i32);
    pub const CurrencyChinese: Self = Self(42i32);
    pub const Bopomofo: Self = Self(43i32);
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
    pub const EmailUserNameOrAddress: Self = Self(60i32);
    pub const Private: Self = Self(61i32);
    pub const Maps: Self = Self(62i32);
    pub const PasswordNumeric: Self = Self(63i32);
    pub const FormulaNumber: Self = Self(67i32);
    pub const ChatWithoutEmoji: Self = Self(68i32);
    pub const Digits: Self = Self(28i32);
    pub const PinNumeric: Self = Self(64i32);
    pub const PinAlphanumeric: Self = Self(65i32);
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
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
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
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
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
