#[cfg(feature = "implement_exclusive")]
pub trait IOcrEngineImpl: Sized {
    fn RecognizeAsync(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<OcrResult>>;
    fn RecognizerLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOcrEngineStaticsImpl: Sized {
    fn MaxImageDimension(&self) -> ::windows::core::Result<u32>;
    fn AvailableRecognizerLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn IsLanguageSupported(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<bool>;
    fn TryCreateFromLanguage(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<OcrEngine>;
    fn TryCreateFromUserProfileLanguages(&self) -> ::windows::core::Result<OcrEngine>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOcrLineImpl: Sized {
    fn Words(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrWord>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOcrResultImpl: Sized {
    fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<OcrLine>>;
    fn TextAngle(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOcrWordImpl: Sized {
    fn BoundingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
