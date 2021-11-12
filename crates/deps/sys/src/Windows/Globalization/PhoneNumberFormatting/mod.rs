#![allow(non_snake_case, non_camel_case_types)]
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
pub struct PhoneNumberFormat(i32);
#[repr(transparent)]
pub struct PhoneNumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberInfo(pub *mut ::core::ffi::c_void);
pub struct PhoneNumberMatchResult(i32);
pub struct PhoneNumberParseResult(i32);
pub struct PredictedPhoneNumberKind(i32);
