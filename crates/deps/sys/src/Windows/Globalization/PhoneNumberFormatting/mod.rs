#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhoneNumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberFormatterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: PhoneNumberFormat = PhoneNumberFormat(0i32);
    pub const International: PhoneNumberFormat = PhoneNumberFormat(1i32);
    pub const National: PhoneNumberFormat = PhoneNumberFormat(2i32);
    pub const Rfc3966: PhoneNumberFormat = PhoneNumberFormat(3i32);
}
#[repr(transparent)]
pub struct PhoneNumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(0i32);
    pub const ShortNationalSignificantNumberMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(1i32);
    pub const NationalSignificantNumberMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(2i32);
    pub const ExactMatch: PhoneNumberMatchResult = PhoneNumberMatchResult(3i32);
}
#[repr(transparent)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: PhoneNumberParseResult = PhoneNumberParseResult(0i32);
    pub const NotANumber: PhoneNumberParseResult = PhoneNumberParseResult(1i32);
    pub const InvalidCountryCode: PhoneNumberParseResult = PhoneNumberParseResult(2i32);
    pub const TooShort: PhoneNumberParseResult = PhoneNumberParseResult(3i32);
    pub const TooLong: PhoneNumberParseResult = PhoneNumberParseResult(4i32);
}
#[repr(transparent)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: PredictedPhoneNumberKind = PredictedPhoneNumberKind(0i32);
    pub const Mobile: PredictedPhoneNumberKind = PredictedPhoneNumberKind(1i32);
    pub const FixedLineOrMobile: PredictedPhoneNumberKind = PredictedPhoneNumberKind(2i32);
    pub const TollFree: PredictedPhoneNumberKind = PredictedPhoneNumberKind(3i32);
    pub const PremiumRate: PredictedPhoneNumberKind = PredictedPhoneNumberKind(4i32);
    pub const SharedCost: PredictedPhoneNumberKind = PredictedPhoneNumberKind(5i32);
    pub const Voip: PredictedPhoneNumberKind = PredictedPhoneNumberKind(6i32);
    pub const PersonalNumber: PredictedPhoneNumberKind = PredictedPhoneNumberKind(7i32);
    pub const Pager: PredictedPhoneNumberKind = PredictedPhoneNumberKind(8i32);
    pub const UniversalAccountNumber: PredictedPhoneNumberKind = PredictedPhoneNumberKind(9i32);
    pub const Voicemail: PredictedPhoneNumberKind = PredictedPhoneNumberKind(10i32);
    pub const Unknown: PredictedPhoneNumberKind = PredictedPhoneNumberKind(11i32);
}
