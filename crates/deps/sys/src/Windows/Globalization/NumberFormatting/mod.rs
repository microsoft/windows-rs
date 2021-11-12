#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrencyFormatter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CurrencyFormatterMode(i32);
#[repr(transparent)]
pub struct DecimalFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyFormatter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrencyFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDecimalFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIncrementNumberRounder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberFormatter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberFormatterOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberParser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberRounder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumberRounderOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumeralSystemTranslator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INumeralSystemTranslatorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPercentFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPermilleFormatterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISignedZeroOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISignificantDigitsOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IncrementNumberRounder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NumeralSystemTranslator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PercentFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PermilleFormatter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RoundingAlgorithm(i32);
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
