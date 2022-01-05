#[cfg(feature = "implement_exclusive")]
pub trait IHtmlUtilitiesImpl: Sized {
    fn ConvertToText(&self, html: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
