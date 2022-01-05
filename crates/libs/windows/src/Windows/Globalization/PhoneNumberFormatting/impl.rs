#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberFormatterImpl: Sized {
    fn Format(&self, number: &::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatWithOutputFormat(&self, number: &::core::option::Option<PhoneNumberInfo>, numberformat: PhoneNumberFormat) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatPartialString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatStringWithLeftToRightMarkers(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberFormatterStaticsImpl: Sized {
    fn TryCreate(&self, regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows::core::Result<()>;
    fn GetCountryCodeForRegion(&self, regioncode: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
    fn GetNationalDirectDialingPrefixForRegion(&self, regioncode: &::windows::core::HSTRING, stripnondigit: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WrapWithLeftToRightMarkers(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoImpl: Sized {
    fn CountryCode(&self) -> ::windows::core::Result<i32>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLengthOfGeographicalAreaCode(&self) -> ::windows::core::Result<i32>;
    fn GetNationalSignificantNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLengthOfNationalDestinationCode(&self) -> ::windows::core::Result<i32>;
    fn PredictNumberKind(&self) -> ::windows::core::Result<PredictedPhoneNumberKind>;
    fn GetGeographicRegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckNumberMatch(&self, othernumber: &::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberMatchResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoFactoryImpl: Sized {
    fn Create(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneNumberInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoStaticsImpl: Sized {
    fn TryParse(&self, input: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult>;
    fn TryParseWithRegion(&self, input: &::windows::core::HSTRING, regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult>;
}
