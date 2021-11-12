#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CurrencyFormatter(i32);
pub struct CurrencyFormatterMode(i32);
pub struct DecimalFormatter(i32);
pub struct ICurrencyFormatter(pub *mut ::core::ffi::c_void);
pub struct ICurrencyFormatter2(pub *mut ::core::ffi::c_void);
pub struct ICurrencyFormatterFactory(pub *mut ::core::ffi::c_void);
pub struct IDecimalFormatterFactory(pub *mut ::core::ffi::c_void);
pub struct IIncrementNumberRounder(pub *mut ::core::ffi::c_void);
pub struct INumberFormatter(pub *mut ::core::ffi::c_void);
pub struct INumberFormatter2(pub *mut ::core::ffi::c_void);
pub struct INumberFormatterOptions(pub *mut ::core::ffi::c_void);
pub struct INumberParser(pub *mut ::core::ffi::c_void);
pub struct INumberRounder(pub *mut ::core::ffi::c_void);
pub struct INumberRounderOption(pub *mut ::core::ffi::c_void);
pub struct INumeralSystemTranslator(pub *mut ::core::ffi::c_void);
pub struct INumeralSystemTranslatorFactory(pub *mut ::core::ffi::c_void);
pub struct IPercentFormatterFactory(pub *mut ::core::ffi::c_void);
pub struct IPermilleFormatterFactory(pub *mut ::core::ffi::c_void);
pub struct ISignedZeroOption(pub *mut ::core::ffi::c_void);
pub struct ISignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
pub struct ISignificantDigitsOption(pub *mut ::core::ffi::c_void);
pub struct IncrementNumberRounder(i32);
pub struct NumeralSystemTranslator(i32);
pub struct PercentFormatter(i32);
pub struct PermilleFormatter(i32);
pub struct RoundingAlgorithm(i32);
pub struct SignificantDigitsNumberRounder(i32);
