#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CurrencyFormatter();
    fn CurrencyFormatterMode();
    fn DecimalFormatter();
    fn ICurrencyFormatter();
    fn ICurrencyFormatter2();
    fn ICurrencyFormatterFactory();
    fn IDecimalFormatterFactory();
    fn IIncrementNumberRounder();
    fn INumberFormatter();
    fn INumberFormatter2();
    fn INumberFormatterOptions();
    fn INumberParser();
    fn INumberRounder();
    fn INumberRounderOption();
    fn INumeralSystemTranslator();
    fn INumeralSystemTranslatorFactory();
    fn IPercentFormatterFactory();
    fn IPermilleFormatterFactory();
    fn ISignedZeroOption();
    fn ISignificantDigitsNumberRounder();
    fn ISignificantDigitsOption();
    fn IncrementNumberRounder();
    fn NumeralSystemTranslator();
    fn PercentFormatter();
    fn PermilleFormatter();
    fn RoundingAlgorithm();
    fn SignificantDigitsNumberRounder();
}
