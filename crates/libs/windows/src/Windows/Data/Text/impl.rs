#[cfg(feature = "implement_exclusive")]
pub trait IAlternateWordFormImpl: Sized {
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
    fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NormalizationFormat(&self) -> ::windows::core::Result<AlternateNormalizationFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordSegmentImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordsSegmenterImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<SelectableWordSegment>;
    fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>;
    fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<SelectableWordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectableWordsSegmenterFactoryImpl: Sized {
    fn CreateWithLanguage(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<SelectableWordsSegmenter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticTextQueryImpl: Sized {
    fn Find(&self, content: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
    fn FindInProperty(&self, propertycontent: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticTextQueryFactoryImpl: Sized {
    fn Create(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
    fn CreateWithLanguage(&self, aqsfilter: &::windows::core::HSTRING, filterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConversionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConversionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPhonemeImpl: Sized {
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadingText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPredictionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPredictionGenerator2Impl: Sized {
    fn GetCandidatesWithParametersAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn GetNextWordCandidatesAsync(&self, maxcandidates: u32, previousstrings: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn InputScope(&self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope>;
    fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextPredictionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextPredictionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextReverseConversionGeneratorImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool>;
    fn ConvertBackAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextReverseConversionGenerator2Impl: Sized {
    fn GetPhonemesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextReverseConversionGeneratorFactoryImpl: Sized {
    fn Create(&self, languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextReverseConversionGenerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnicodeCharactersStaticsImpl: Sized {
    fn GetCodepointFromSurrogatePair(&self, highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32>;
    fn GetSurrogatePairFromCodepoint(&self, codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()>;
    fn IsHighSurrogate(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowSurrogate(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsSupplementary(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsNoncharacter(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsWhitespace(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsAlphabetic(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsCased(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsUppercase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsLowercase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdStart(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsIdContinue(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeBase(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn IsGraphemeExtend(&self, codepoint: u32) -> ::windows::core::Result<bool>;
    fn GetNumericType(&self, codepoint: u32) -> ::windows::core::Result<UnicodeNumericType>;
    fn GetGeneralCategory(&self, codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWordSegmentImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment>;
    fn AlternateForms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWordsSegmenterImpl: Sized {
    fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<WordSegment>;
    fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>>;
    fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &::core::option::Option<WordSegmentsTokenizingHandler>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWordsSegmenterFactoryImpl: Sized {
    fn CreateWithLanguage(&self, language: &::windows::core::HSTRING) -> ::windows::core::Result<WordsSegmenter>;
}
