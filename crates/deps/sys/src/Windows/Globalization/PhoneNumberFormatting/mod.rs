#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPhoneNumberFormatter(pub *mut ::core::ffi::c_void);
pub struct IPhoneNumberFormatterStatics(pub *mut ::core::ffi::c_void);
pub struct IPhoneNumberInfo(pub *mut ::core::ffi::c_void);
pub struct IPhoneNumberInfoFactory(pub *mut ::core::ffi::c_void);
pub struct IPhoneNumberInfoStatics(pub *mut ::core::ffi::c_void);
pub struct PhoneNumberFormat(i32);
pub struct PhoneNumberFormatter(i32);
pub struct PhoneNumberInfo(i32);
pub struct PhoneNumberMatchResult(i32);
pub struct PhoneNumberParseResult(i32);
pub struct PredictedPhoneNumberKind(i32);
