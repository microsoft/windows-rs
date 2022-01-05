#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontImpl: Sized {
    fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FontWeight(&self) -> ::windows::core::Result<super::super::UI::Text::FontWeight>;
    fn FontStretch(&self) -> ::windows::core::Result<super::super::UI::Text::FontStretch>;
    fn FontStyle(&self) -> ::windows::core::Result<super::super::UI::Text::FontStyle>;
    fn ScaleFactor(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroupImpl: Sized {
    fn UITextFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UIHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UITitleFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UICaptionFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn UINotificationHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn TraditionalDocumentFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn ModernDocumentFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentHeadingFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn FixedWidthTextFont(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate1Font(&self) -> ::windows::core::Result<LanguageFont>;
    fn DocumentAlternate2Font(&self) -> ::windows::core::Result<LanguageFont>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILanguageFontGroupFactoryImpl: Sized {
    fn CreateLanguageFontGroup(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<LanguageFontGroup>;
}
