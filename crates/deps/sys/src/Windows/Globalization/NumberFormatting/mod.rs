#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrencyFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrencyFormatterMode(pub i32);
impl CurrencyFormatterMode {
    pub const UseSymbol: CurrencyFormatterMode = CurrencyFormatterMode(0i32);
    pub const UseCurrencyCode: CurrencyFormatterMode = CurrencyFormatterMode(1i32);
}
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
#[repr(transparent)]
pub struct RoundingAlgorithm(pub i32);
impl RoundingAlgorithm {
    pub const None: RoundingAlgorithm = RoundingAlgorithm(0i32);
    pub const RoundDown: RoundingAlgorithm = RoundingAlgorithm(1i32);
    pub const RoundUp: RoundingAlgorithm = RoundingAlgorithm(2i32);
    pub const RoundTowardsZero: RoundingAlgorithm = RoundingAlgorithm(3i32);
    pub const RoundAwayFromZero: RoundingAlgorithm = RoundingAlgorithm(4i32);
    pub const RoundHalfDown: RoundingAlgorithm = RoundingAlgorithm(5i32);
    pub const RoundHalfUp: RoundingAlgorithm = RoundingAlgorithm(6i32);
    pub const RoundHalfTowardsZero: RoundingAlgorithm = RoundingAlgorithm(7i32);
    pub const RoundHalfAwayFromZero: RoundingAlgorithm = RoundingAlgorithm(8i32);
    pub const RoundHalfToEven: RoundingAlgorithm = RoundingAlgorithm(9i32);
    pub const RoundHalfToOdd: RoundingAlgorithm = RoundingAlgorithm(10i32);
}
#[repr(transparent)]
pub struct SignificantDigitsNumberRounder(pub *mut ::core::ffi::c_void);
