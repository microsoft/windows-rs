#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatterImpl: Sized + INumberFormatterImpl + INumberFormatter2Impl + INumberFormatterOptionsImpl + INumberParserImpl {
    fn Currency(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrency(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatter2Impl: Sized {
    fn Mode(&self) -> ::windows::core::Result<CurrencyFormatterMode>;
    fn SetMode(&self, value: CurrencyFormatterMode) -> ::windows::core::Result<()>;
    fn ApplyRoundingForCurrency(&self, roundingalgorithm: RoundingAlgorithm) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrencyFormatterFactoryImpl: Sized {
    fn CreateCurrencyFormatterCode(&self, currencycode: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
    fn CreateCurrencyFormatterCodeContext(&self, currencycode: &::windows::core::HSTRING, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<CurrencyFormatter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDecimalFormatterFactoryImpl: Sized {
    fn CreateDecimalFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<DecimalFormatter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIncrementNumberRounderImpl: Sized {
    fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm>;
    fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()>;
    fn Increment(&self) -> ::windows::core::Result<f64>;
    fn SetIncrement(&self, value: f64) -> ::windows::core::Result<()>;
}
pub trait INumberFormatterImpl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait INumberFormatter2Impl: Sized {
    fn FormatInt(&self, value: i64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait INumberFormatterOptionsImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntegerDigits(&self) -> ::windows::core::Result<i32>;
    fn SetIntegerDigits(&self, value: i32) -> ::windows::core::Result<()>;
    fn FractionDigits(&self) -> ::windows::core::Result<i32>;
    fn SetFractionDigits(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsGrouped(&self) -> ::windows::core::Result<bool>;
    fn SetIsGrouped(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecimalPointAlwaysDisplayed(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecimalPointAlwaysDisplayed(&self, value: bool) -> ::windows::core::Result<()>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ResolvedGeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait INumberParserImpl: Sized {
    fn ParseInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ParseUInt(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<u64>>;
    fn ParseDouble(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
pub trait INumberRounderImpl: Sized {
    fn RoundInt32(&self, value: i32) -> ::windows::core::Result<i32>;
    fn RoundUInt32(&self, value: u32) -> ::windows::core::Result<u32>;
    fn RoundInt64(&self, value: i64) -> ::windows::core::Result<i64>;
    fn RoundUInt64(&self, value: u64) -> ::windows::core::Result<u64>;
    fn RoundSingle(&self, value: f32) -> ::windows::core::Result<f32>;
    fn RoundDouble(&self, value: f64) -> ::windows::core::Result<f64>;
}
pub trait INumberRounderOptionImpl: Sized {
    fn NumberRounder(&self) -> ::windows::core::Result<INumberRounder>;
    fn SetNumberRounder(&self, value: &::core::option::Option<INumberRounder>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemTranslatorImpl: Sized {
    fn Languages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NumeralSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNumeralSystem(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TranslateNumerals(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INumeralSystemTranslatorFactoryImpl: Sized {
    fn Create(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<NumeralSystemTranslator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPercentFormatterFactoryImpl: Sized {
    fn CreatePercentFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PercentFormatter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPermilleFormatterFactoryImpl: Sized {
    fn CreatePermilleFormatter(&self, languages: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, geographicregion: &::windows::core::HSTRING) -> ::windows::core::Result<PermilleFormatter>;
}
pub trait ISignedZeroOptionImpl: Sized {
    fn IsZeroSigned(&self) -> ::windows::core::Result<bool>;
    fn SetIsZeroSigned(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISignificantDigitsNumberRounderImpl: Sized {
    fn RoundingAlgorithm(&self) -> ::windows::core::Result<RoundingAlgorithm>;
    fn SetRoundingAlgorithm(&self, value: RoundingAlgorithm) -> ::windows::core::Result<()>;
    fn SignificantDigits(&self) -> ::windows::core::Result<u32>;
    fn SetSignificantDigits(&self, value: u32) -> ::windows::core::Result<()>;
}
pub trait ISignificantDigitsOptionImpl: Sized {
    fn SignificantDigits(&self) -> ::windows::core::Result<i32>;
    fn SetSignificantDigits(&self, value: i32) -> ::windows::core::Result<()>;
}
