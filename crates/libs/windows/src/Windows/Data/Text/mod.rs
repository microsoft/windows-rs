#[doc(hidden)]
#[repr(transparent)]
pub struct IAlternateWordForm(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlternateWordForm {
    type Vtable = IAlternateWordForm_Vtbl;
}
impl ::core::clone::Clone for IAlternateWordForm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAlternateWordForm {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47396c1e_51b9_4207_9146_248e636a1d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlternateWordForm_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    pub AlternateText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NormalizationFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlternateNormalizationFormat) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordSegment {
    type Vtable = ISelectableWordSegment_Vtbl;
}
impl ::core::clone::Clone for ISelectableWordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISelectableWordSegment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x916a4cb7_8aa7_4c78_b374_5dedb752e60b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordSegment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordsSegmenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_Vtbl;
}
impl ::core::clone::Clone for ISelectableWordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISelectableWordsSegmenter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6dc31e7_4b13_45c5_8897_7d71269e085d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, startindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, startindex: u32, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectableWordsSegmenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISelectableWordsSegmenterFactory {
    type Vtable = ISelectableWordsSegmenterFactory_Vtbl;
}
impl ::core::clone::Clone for ISelectableWordsSegmenterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISelectableWordsSegmenterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c7a7648_6057_4339_bc70_f210010a4150);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectableWordsSegmenterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISemanticTextQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISemanticTextQuery {
    type Vtable = ISemanticTextQuery_Vtbl;
}
impl ::core::clone::Clone for ISemanticTextQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISemanticTextQuery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a1cab51_1fb2_4909_80b8_35731a2b3e7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQuery_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Find: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Find: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindInProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycontent: ::std::mem::MaybeUninit<::windows::core::HSTRING>, propertyname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindInProperty: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISemanticTextQueryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISemanticTextQueryFactory {
    type Vtable = ISemanticTextQueryFactory_Vtbl;
}
impl ::core::clone::Clone for ISemanticTextQueryFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISemanticTextQueryFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x238c0503_f995_4587_8777_a2b7d80acfef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISemanticTextQueryFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows::core::HSTRING>, filterlanguage: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextConversionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextConversionGenerator {
    type Vtable = ITextConversionGenerator_Vtbl;
}
impl ::core::clone::Clone for ITextConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextConversionGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03606a5e_2aa9_4ab6_af8b_a562b63a8992);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGenerator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextConversionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextConversionGeneratorFactory {
    type Vtable = ITextConversionGeneratorFactory_Vtbl;
}
impl ::core::clone::Clone for ITextConversionGeneratorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextConversionGeneratorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcaa3781_3083_49ab_be15_56dfbbb74d6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConversionGeneratorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPhoneme(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPhoneme {
    type Vtable = ITextPhoneme_Vtbl;
}
impl ::core::clone::Clone for ITextPhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextPhoneme {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9362a40a_9b7a_4569_94cf_d84f2f38cf9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPhoneme_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReadingText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_Vtbl;
}
impl ::core::clone::Clone for ITextPredictionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextPredictionGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eacab07_abf1_4cb6_9d9e_326f2b468756);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithMaxCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, maxcandidates: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithMaxCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGenerator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGenerator2 {
    type Vtable = ITextPredictionGenerator2_Vtbl;
}
impl ::core::clone::Clone for ITextPredictionGenerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextPredictionGenerator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84723b8_2c77_486a_900a_a3453eedc15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGenerator2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCandidatesWithParametersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCandidatesWithParametersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNextWordCandidatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxcandidates: u32, previousstrings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNextWordCandidatesAsync: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub InputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    InputScope: usize,
    #[cfg(feature = "UI_Text_Core")]
    pub SetInputScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text_Core"))]
    SetInputScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextPredictionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextPredictionGeneratorFactory {
    type Vtable = ITextPredictionGeneratorFactory_Vtbl;
}
impl ::core::clone::Clone for ITextPredictionGeneratorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextPredictionGeneratorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7257b416_8ba2_4751_9d30_9d85435653a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextPredictionGeneratorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGenerator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_Vtbl;
}
impl ::core::clone::Clone for ITextReverseConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextReverseConversionGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e7f514_9c51_4d86_ae1b_b498fbad8313);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LanguageAvailableButNotInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConvertBackAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConvertBackAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGenerator2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGenerator2 {
    type Vtable = ITextReverseConversionGenerator2_Vtbl;
}
impl ::core::clone::Clone for ITextReverseConversionGenerator2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextReverseConversionGenerator2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aafd2ec_85d6_46fd_828a_3a4830fa6e18);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGenerator2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPhonemesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPhonemesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITextReverseConversionGeneratorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITextReverseConversionGeneratorFactory {
    type Vtable = ITextReverseConversionGeneratorFactory_Vtbl;
}
impl ::core::clone::Clone for ITextReverseConversionGeneratorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextReverseConversionGeneratorFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bed326_1fda_41f6_89d5_23ddea3c729a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextReverseConversionGeneratorFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languagetag: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnicodeCharactersStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnicodeCharactersStatics {
    type Vtable = IUnicodeCharactersStatics_Vtbl;
}
impl ::core::clone::Clone for IUnicodeCharactersStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUnicodeCharactersStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97909e87_9291_4f91_b6c8_b6e359d7a7fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnicodeCharactersStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCodepointFromSurrogatePair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, highsurrogate: u32, lowsurrogate: u32, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetSurrogatePairFromCodepoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, highsurrogate: *mut u16, lowsurrogate: *mut u16) -> ::windows::core::HRESULT,
    pub IsHighSurrogate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLowSurrogate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSupplementary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsNoncharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAlphabetic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUppercase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLowercase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsIdStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsIdContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGraphemeBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGraphemeExtend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetNumericType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeNumericType) -> ::windows::core::HRESULT,
    pub GetGeneralCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoint: u32, result__: *mut UnicodeGeneralCategory) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordSegment {
    type Vtable = IWordSegment_Vtbl;
}
impl ::core::clone::Clone for IWordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWordSegment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d4ba6d_987c_4cc0_b6bd_d49a11b38f9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordSegment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SourceTextSegment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TextSegment) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateForms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateForms: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordsSegmenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordsSegmenter {
    type Vtable = IWordsSegmenter_Vtbl;
}
impl ::core::clone::Clone for IWordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWordsSegmenter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86b4d4d1_b2fe_4e34_a81d_66640300454f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResolvedLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetTokenAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, startindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTokens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTokens: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Tokenize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, startindex: u32, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tokenize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWordsSegmenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWordsSegmenterFactory {
    type Vtable = IWordsSegmenterFactory_Vtbl;
}
impl ::core::clone::Clone for IWordsSegmenterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWordsSegmenterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6977274_fc35_455c_8bfb_6d7f4653ca97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWordsSegmenterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct AlternateWordForm(::windows::core::IUnknown);
impl AlternateWordForm {
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TextSegment>();
            (::windows::core::Interface::vtable(this).SourceTextSegment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AlternateText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AlternateText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NormalizationFormat(&self) -> ::windows::core::Result<AlternateNormalizationFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AlternateNormalizationFormat>();
            (::windows::core::Interface::vtable(this).NormalizationFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AlternateWordForm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AlternateWordForm {}
impl ::core::fmt::Debug for AlternateWordForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlternateWordForm").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AlternateWordForm {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.AlternateWordForm;{47396c1e-51b9-4207-9146-248e636a1d1d})");
}
impl ::core::clone::Clone for AlternateWordForm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AlternateWordForm {
    type Vtable = IAlternateWordForm_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AlternateWordForm {
    const IID: ::windows::core::GUID = <IAlternateWordForm as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AlternateWordForm {
    const NAME: &'static str = "Windows.Data.Text.AlternateWordForm";
}
::windows::imp::interface_hierarchy!(AlternateWordForm, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AlternateWordForm {}
unsafe impl ::core::marker::Sync for AlternateWordForm {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct SelectableWordSegment(::windows::core::IUnknown);
impl SelectableWordSegment {
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TextSegment>();
            (::windows::core::Interface::vtable(this).SourceTextSegment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SelectableWordSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectableWordSegment {}
impl ::core::fmt::Debug for SelectableWordSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectableWordSegment").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SelectableWordSegment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordSegment;{916a4cb7-8aa7-4c78-b374-5dedb752e60b})");
}
impl ::core::clone::Clone for SelectableWordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SelectableWordSegment {
    type Vtable = ISelectableWordSegment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SelectableWordSegment {
    const IID: ::windows::core::GUID = <ISelectableWordSegment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SelectableWordSegment {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordSegment";
}
::windows::imp::interface_hierarchy!(SelectableWordSegment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SelectableWordSegment {}
unsafe impl ::core::marker::Sync for SelectableWordSegment {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct SelectableWordsSegmenter(::windows::core::IUnknown);
impl SelectableWordsSegmenter {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<SelectableWordSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SelectableWordSegment>();
            (::windows::core::Interface::vtable(this).GetTokenAt)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), startindex, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<SelectableWordSegment>>();
            (::windows::core::Interface::vtable(this).GetTokens)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &SelectableWordSegmentsTokenizingHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Tokenize)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), startindex, ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn CreateWithLanguage(language: &::windows::core::HSTRING) -> ::windows::core::Result<SelectableWordsSegmenter> {
        Self::ISelectableWordsSegmenterFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SelectableWordsSegmenter>();
            (::windows::core::Interface::vtable(this).CreateWithLanguage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(language), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISelectableWordsSegmenterFactory<R, F: FnOnce(&ISelectableWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SelectableWordsSegmenter, ISelectableWordsSegmenterFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SelectableWordsSegmenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectableWordsSegmenter {}
impl ::core::fmt::Debug for SelectableWordsSegmenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectableWordsSegmenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SelectableWordsSegmenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SelectableWordsSegmenter;{f6dc31e7-4b13-45c5-8897-7d71269e085d})");
}
impl ::core::clone::Clone for SelectableWordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SelectableWordsSegmenter {
    type Vtable = ISelectableWordsSegmenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SelectableWordsSegmenter {
    const IID: ::windows::core::GUID = <ISelectableWordsSegmenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SelectableWordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.SelectableWordsSegmenter";
}
::windows::imp::interface_hierarchy!(SelectableWordsSegmenter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SelectableWordsSegmenter {}
unsafe impl ::core::marker::Sync for SelectableWordsSegmenter {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct SemanticTextQuery(::windows::core::IUnknown);
impl SemanticTextQuery {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Find(&self, content: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<TextSegment>>();
            (::windows::core::Interface::vtable(this).Find)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindInProperty(&self, propertycontent: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TextSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<TextSegment>>();
            (::windows::core::Interface::vtable(this).FindInProperty)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(propertycontent), ::core::mem::transmute_copy(propertyname), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SemanticTextQuery>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithLanguage(aqsfilter: &::windows::core::HSTRING, filterlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<SemanticTextQuery> {
        Self::ISemanticTextQueryFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SemanticTextQuery>();
            (::windows::core::Interface::vtable(this).CreateWithLanguage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), ::core::mem::transmute_copy(filterlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISemanticTextQueryFactory<R, F: FnOnce(&ISemanticTextQueryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SemanticTextQuery, ISemanticTextQueryFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SemanticTextQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SemanticTextQuery {}
impl ::core::fmt::Debug for SemanticTextQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SemanticTextQuery").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SemanticTextQuery {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.SemanticTextQuery;{6a1cab51-1fb2-4909-80b8-35731a2b3e7f})");
}
impl ::core::clone::Clone for SemanticTextQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SemanticTextQuery {
    type Vtable = ISemanticTextQuery_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SemanticTextQuery {
    const IID: ::windows::core::GUID = <ISemanticTextQuery as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SemanticTextQuery {
    const NAME: &'static str = "Windows.Data.Text.SemanticTextQuery";
}
::windows::imp::interface_hierarchy!(SemanticTextQuery, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SemanticTextQuery {}
unsafe impl ::core::marker::Sync for SemanticTextQuery {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct TextConversionGenerator(::windows::core::IUnknown);
impl TextConversionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).LanguageAvailableButNotInstalled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetCandidatesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetCandidatesWithMaxCountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), maxcandidates, &mut result__).from_abi(result__)
        }
    }
    pub fn Create(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextConversionGenerator> {
        Self::ITextConversionGeneratorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TextConversionGenerator>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextConversionGeneratorFactory<R, F: FnOnce(&ITextConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TextConversionGenerator, ITextConversionGeneratorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TextConversionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextConversionGenerator {}
impl ::core::fmt::Debug for TextConversionGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextConversionGenerator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TextConversionGenerator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextConversionGenerator;{03606a5e-2aa9-4ab6-af8b-a562b63a8992})");
}
impl ::core::clone::Clone for TextConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TextConversionGenerator {
    type Vtable = ITextConversionGenerator_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TextConversionGenerator {
    const IID: ::windows::core::GUID = <ITextConversionGenerator as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TextConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextConversionGenerator";
}
::windows::imp::interface_hierarchy!(TextConversionGenerator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TextConversionGenerator {}
unsafe impl ::core::marker::Sync for TextConversionGenerator {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct TextPhoneme(::windows::core::IUnknown);
impl TextPhoneme {
    pub fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadingText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ReadingText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TextPhoneme {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPhoneme {}
impl ::core::fmt::Debug for TextPhoneme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPhoneme").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TextPhoneme {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPhoneme;{9362a40a-9b7a-4569-94cf-d84f2f38cf9b})");
}
impl ::core::clone::Clone for TextPhoneme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TextPhoneme {
    type Vtable = ITextPhoneme_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TextPhoneme {
    const IID: ::windows::core::GUID = <ITextPhoneme as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TextPhoneme {
    const NAME: &'static str = "Windows.Data.Text.TextPhoneme";
}
::windows::imp::interface_hierarchy!(TextPhoneme, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TextPhoneme {}
unsafe impl ::core::marker::Sync for TextPhoneme {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct TextPredictionGenerator(::windows::core::IUnknown);
impl TextPredictionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).LanguageAvailableButNotInstalled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetCandidatesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithMaxCountAsync(&self, input: &::windows::core::HSTRING, maxcandidates: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetCandidatesWithMaxCountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), maxcandidates, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCandidatesWithParametersAsync<P0>(&self, input: &::windows::core::HSTRING, maxcandidates: u32, predictionoptions: TextPredictionOptions, previousstrings: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetCandidatesWithParametersAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), maxcandidates, predictionoptions, previousstrings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNextWordCandidatesAsync<P0>(&self, maxcandidates: u32, previousstrings: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>();
            (::windows::core::Interface::vtable(this).GetNextWordCandidatesAsync)(::windows::core::Interface::as_raw(this), maxcandidates, previousstrings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    #[cfg(feature = "UI_Text_Core")]
    pub fn InputScope(&self) -> ::windows::core::Result<super::super::UI::Text::Core::CoreTextInputScope> {
        let this = &::windows::core::ComInterface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Text::Core::CoreTextInputScope>();
            (::windows::core::Interface::vtable(this).InputScope)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Text_Core\"`*"]
    #[cfg(feature = "UI_Text_Core")]
    pub fn SetInputScope(&self, value: super::super::UI::Text::Core::CoreTextInputScope) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<ITextPredictionGenerator2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetInputScope)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextPredictionGenerator> {
        Self::ITextPredictionGeneratorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TextPredictionGenerator>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextPredictionGeneratorFactory<R, F: FnOnce(&ITextPredictionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TextPredictionGenerator, ITextPredictionGeneratorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TextPredictionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPredictionGenerator {}
impl ::core::fmt::Debug for TextPredictionGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPredictionGenerator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TextPredictionGenerator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextPredictionGenerator;{5eacab07-abf1-4cb6-9d9e-326f2b468756})");
}
impl ::core::clone::Clone for TextPredictionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TextPredictionGenerator {
    type Vtable = ITextPredictionGenerator_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TextPredictionGenerator {
    const IID: ::windows::core::GUID = <ITextPredictionGenerator as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TextPredictionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextPredictionGenerator";
}
::windows::imp::interface_hierarchy!(TextPredictionGenerator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TextPredictionGenerator {}
unsafe impl ::core::marker::Sync for TextPredictionGenerator {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct TextReverseConversionGenerator(::windows::core::IUnknown);
impl TextReverseConversionGenerator {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LanguageAvailableButNotInstalled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).LanguageAvailableButNotInstalled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConvertBackAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).ConvertBackAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPhonemesAsync(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>> {
        let this = &::windows::core::ComInterface::cast::<ITextReverseConversionGenerator2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<TextPhoneme>>>();
            (::windows::core::Interface::vtable(this).GetPhonemesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(input), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(languagetag: &::windows::core::HSTRING) -> ::windows::core::Result<TextReverseConversionGenerator> {
        Self::ITextReverseConversionGeneratorFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TextReverseConversionGenerator>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(languagetag), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITextReverseConversionGeneratorFactory<R, F: FnOnce(&ITextReverseConversionGeneratorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TextReverseConversionGenerator, ITextReverseConversionGeneratorFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TextReverseConversionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextReverseConversionGenerator {}
impl ::core::fmt::Debug for TextReverseConversionGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextReverseConversionGenerator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TextReverseConversionGenerator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.TextReverseConversionGenerator;{51e7f514-9c51-4d86-ae1b-b498fbad8313})");
}
impl ::core::clone::Clone for TextReverseConversionGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TextReverseConversionGenerator {
    type Vtable = ITextReverseConversionGenerator_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TextReverseConversionGenerator {
    const IID: ::windows::core::GUID = <ITextReverseConversionGenerator as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TextReverseConversionGenerator {
    const NAME: &'static str = "Windows.Data.Text.TextReverseConversionGenerator";
}
::windows::imp::interface_hierarchy!(TextReverseConversionGenerator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TextReverseConversionGenerator {}
unsafe impl ::core::marker::Sync for TextReverseConversionGenerator {}
#[doc = "*Required features: `\"Data_Text\"`*"]
pub struct UnicodeCharacters;
impl UnicodeCharacters {
    pub fn GetCodepointFromSurrogatePair(highsurrogate: u32, lowsurrogate: u32) -> ::windows::core::Result<u32> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetCodepointFromSurrogatePair)(::windows::core::Interface::as_raw(this), highsurrogate, lowsurrogate, &mut result__).from_abi(result__)
        })
    }
    pub fn GetSurrogatePairFromCodepoint(codepoint: u32, highsurrogate: &mut u16, lowsurrogate: &mut u16) -> ::windows::core::Result<()> {
        Self::IUnicodeCharactersStatics(|this| unsafe { (::windows::core::Interface::vtable(this).GetSurrogatePairFromCodepoint)(::windows::core::Interface::as_raw(this), codepoint, highsurrogate, lowsurrogate).ok() })
    }
    pub fn IsHighSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsHighSurrogate)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsLowSurrogate(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsLowSurrogate)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupplementary(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSupplementary)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsNoncharacter(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsNoncharacter)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsWhitespace(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWhitespace)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsAlphabetic(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAlphabetic)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsCased(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsCased)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsUppercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsUppercase)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsLowercase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsLowercase)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsIdStart(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsIdStart)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsIdContinue(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsIdContinue)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsGraphemeBase(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsGraphemeBase)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn IsGraphemeExtend(codepoint: u32) -> ::windows::core::Result<bool> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsGraphemeExtend)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn GetNumericType(codepoint: u32) -> ::windows::core::Result<UnicodeNumericType> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UnicodeNumericType>();
            (::windows::core::Interface::vtable(this).GetNumericType)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    pub fn GetGeneralCategory(codepoint: u32) -> ::windows::core::Result<UnicodeGeneralCategory> {
        Self::IUnicodeCharactersStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UnicodeGeneralCategory>();
            (::windows::core::Interface::vtable(this).GetGeneralCategory)(::windows::core::Interface::as_raw(this), codepoint, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUnicodeCharactersStatics<R, F: FnOnce(&IUnicodeCharactersStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UnicodeCharacters, IUnicodeCharactersStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for UnicodeCharacters {
    const NAME: &'static str = "Windows.Data.Text.UnicodeCharacters";
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct WordSegment(::windows::core::IUnknown);
impl WordSegment {
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceTextSegment(&self) -> ::windows::core::Result<TextSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TextSegment>();
            (::windows::core::Interface::vtable(this).SourceTextSegment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateForms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AlternateWordForm>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<AlternateWordForm>>();
            (::windows::core::Interface::vtable(this).AlternateForms)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WordSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WordSegment {}
impl ::core::fmt::Debug for WordSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WordSegment").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WordSegment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordSegment;{d2d4ba6d-987c-4cc0-b6bd-d49a11b38f9a})");
}
impl ::core::clone::Clone for WordSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WordSegment {
    type Vtable = IWordSegment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WordSegment {
    const IID: ::windows::core::GUID = <IWordSegment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WordSegment {
    const NAME: &'static str = "Windows.Data.Text.WordSegment";
}
::windows::imp::interface_hierarchy!(WordSegment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WordSegment {}
unsafe impl ::core::marker::Sync for WordSegment {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
pub struct WordsSegmenter(::windows::core::IUnknown);
impl WordsSegmenter {
    pub fn ResolvedLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResolvedLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetTokenAt(&self, text: &::windows::core::HSTRING, startindex: u32) -> ::windows::core::Result<WordSegment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WordSegment>();
            (::windows::core::Interface::vtable(this).GetTokenAt)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), startindex, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTokens(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WordSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<WordSegment>>();
            (::windows::core::Interface::vtable(this).GetTokens)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tokenize(&self, text: &::windows::core::HSTRING, startindex: u32, handler: &WordSegmentsTokenizingHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Tokenize)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), startindex, ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn CreateWithLanguage(language: &::windows::core::HSTRING) -> ::windows::core::Result<WordsSegmenter> {
        Self::IWordsSegmenterFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WordsSegmenter>();
            (::windows::core::Interface::vtable(this).CreateWithLanguage)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(language), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWordsSegmenterFactory<R, F: FnOnce(&IWordsSegmenterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WordsSegmenter, IWordsSegmenterFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WordsSegmenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WordsSegmenter {}
impl ::core::fmt::Debug for WordsSegmenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WordsSegmenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WordsSegmenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Data.Text.WordsSegmenter;{86b4d4d1-b2fe-4e34-a81d-66640300454f})");
}
impl ::core::clone::Clone for WordsSegmenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WordsSegmenter {
    type Vtable = IWordsSegmenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WordsSegmenter {
    const IID: ::windows::core::GUID = <IWordsSegmenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WordsSegmenter {
    const NAME: &'static str = "Windows.Data.Text.WordsSegmenter";
}
::windows::imp::interface_hierarchy!(WordsSegmenter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WordsSegmenter {}
unsafe impl ::core::marker::Sync for WordsSegmenter {}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Currency: Self = Self(3i32);
    pub const Date: Self = Self(4i32);
    pub const Time: Self = Self(5i32);
}
impl ::core::marker::Copy for AlternateNormalizationFormat {}
impl ::core::clone::Clone for AlternateNormalizationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlternateNormalizationFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AlternateNormalizationFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AlternateNormalizationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlternateNormalizationFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AlternateNormalizationFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.AlternateNormalizationFormat;i4)");
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: Self = Self(0u32);
    pub const Predictions: Self = Self(1u32);
    pub const Corrections: Self = Self(2u32);
}
impl ::core::marker::Copy for TextPredictionOptions {}
impl ::core::clone::Clone for TextPredictionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TextPredictionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TextPredictionOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TextPredictionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPredictionOptions").field(&self.0).finish()
    }
}
impl TextPredictionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TextPredictionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TextPredictionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TextPredictionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TextPredictionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TextPredictionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for TextPredictionOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.TextPredictionOptions;u4)");
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: Self = Self(0i32);
    pub const LowercaseLetter: Self = Self(1i32);
    pub const TitlecaseLetter: Self = Self(2i32);
    pub const ModifierLetter: Self = Self(3i32);
    pub const OtherLetter: Self = Self(4i32);
    pub const NonspacingMark: Self = Self(5i32);
    pub const SpacingCombiningMark: Self = Self(6i32);
    pub const EnclosingMark: Self = Self(7i32);
    pub const DecimalDigitNumber: Self = Self(8i32);
    pub const LetterNumber: Self = Self(9i32);
    pub const OtherNumber: Self = Self(10i32);
    pub const SpaceSeparator: Self = Self(11i32);
    pub const LineSeparator: Self = Self(12i32);
    pub const ParagraphSeparator: Self = Self(13i32);
    pub const Control: Self = Self(14i32);
    pub const Format: Self = Self(15i32);
    pub const Surrogate: Self = Self(16i32);
    pub const PrivateUse: Self = Self(17i32);
    pub const ConnectorPunctuation: Self = Self(18i32);
    pub const DashPunctuation: Self = Self(19i32);
    pub const OpenPunctuation: Self = Self(20i32);
    pub const ClosePunctuation: Self = Self(21i32);
    pub const InitialQuotePunctuation: Self = Self(22i32);
    pub const FinalQuotePunctuation: Self = Self(23i32);
    pub const OtherPunctuation: Self = Self(24i32);
    pub const MathSymbol: Self = Self(25i32);
    pub const CurrencySymbol: Self = Self(26i32);
    pub const ModifierSymbol: Self = Self(27i32);
    pub const OtherSymbol: Self = Self(28i32);
    pub const NotAssigned: Self = Self(29i32);
}
impl ::core::marker::Copy for UnicodeGeneralCategory {}
impl ::core::clone::Clone for UnicodeGeneralCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnicodeGeneralCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UnicodeGeneralCategory {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UnicodeGeneralCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnicodeGeneralCategory").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UnicodeGeneralCategory {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeGeneralCategory;i4)");
}
#[doc = "*Required features: `\"Data_Text\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: Self = Self(0i32);
    pub const Decimal: Self = Self(1i32);
    pub const Digit: Self = Self(2i32);
    pub const Numeric: Self = Self(3i32);
}
impl ::core::marker::Copy for UnicodeNumericType {}
impl ::core::clone::Clone for UnicodeNumericType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnicodeNumericType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UnicodeNumericType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UnicodeNumericType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnicodeNumericType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UnicodeNumericType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Text.UnicodeNumericType;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Data_Text\"`*"]
pub struct TextSegment {
    pub StartPosition: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for TextSegment {}
impl ::core::clone::Clone for TextSegment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TextSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TextSegment").field("StartPosition", &self.StartPosition).field("Length", &self.Length).finish()
    }
}
impl ::windows::core::TypeKind for TextSegment {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for TextSegment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Data.Text.TextSegment;u4;u4)");
}
impl ::core::cmp::PartialEq for TextSegment {
    fn eq(&self, other: &Self) -> bool {
        self.StartPosition == other.StartPosition && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for TextSegment {}
impl ::core::default::Default for TextSegment {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SelectableWordSegmentsTokenizingHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SelectableWordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SelectableWordSegmentsTokenizingHandlerBox::<F> { vtable: &SelectableWordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, P1>(&self, precedingwords: P0, words: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<SelectableWordSegment>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), precedingwords.try_into_param()?.abi(), words.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct SelectableWordSegmentsTokenizingHandlerBox<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SelectableWordSegmentsTokenizingHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<SelectableWordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SelectableWordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: SelectableWordSegmentsTokenizingHandler_Vtbl = SelectableWordSegmentsTokenizingHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SelectableWordSegmentsTokenizingHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, precedingwords: *mut ::core::ffi::c_void, words: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&precedingwords), ::windows::core::from_raw_borrowed(&words)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for SelectableWordSegmentsTokenizingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for SelectableWordSegmentsTokenizingHandler {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for SelectableWordSegmentsTokenizingHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectableWordSegmentsTokenizingHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for SelectableWordSegmentsTokenizingHandler {
    type Vtable = SelectableWordSegmentsTokenizingHandler_Vtbl;
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SelectableWordSegmentsTokenizingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::ComInterface for SelectableWordSegmentsTokenizingHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a3dfc9c_aede_4dc7_9e6c_41c044bd3592);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeType for SelectableWordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{3a3dfc9c-aede-4dc7-9e6c-41c044bd3592}");
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct SelectableWordSegmentsTokenizingHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precedingwords: *mut ::core::ffi::c_void, words: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct WordSegmentsTokenizingHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl WordSegmentsTokenizingHandler {
    pub fn new<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WordSegmentsTokenizingHandlerBox::<F> { vtable: &WordSegmentsTokenizingHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invoke<P0, P1>(&self, precedingwords: P0, words: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<WordSegment>>,
        P1: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<WordSegment>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), precedingwords.try_into_param()?.abi(), words.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
struct WordSegmentsTokenizingHandlerBox<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WordSegmentsTokenizingHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
#[cfg(feature = "Foundation_Collections")]
impl<F: FnMut(::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>, ::core::option::Option<&super::super::Foundation::Collections::IIterable<WordSegment>>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WordSegmentsTokenizingHandlerBox<F> {
    const VTABLE: WordSegmentsTokenizingHandler_Vtbl = WordSegmentsTokenizingHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<WordSegmentsTokenizingHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, precedingwords: *mut ::core::ffi::c_void, words: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&precedingwords), ::windows::core::from_raw_borrowed(&words)).into()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for WordSegmentsTokenizingHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for WordSegmentsTokenizingHandler {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for WordSegmentsTokenizingHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WordSegmentsTokenizingHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for WordSegmentsTokenizingHandler {
    type Vtable = WordSegmentsTokenizingHandler_Vtbl;
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for WordSegmentsTokenizingHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::ComInterface for WordSegmentsTokenizingHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5dd6357_bf2a_4c4f_a31f_29e71c6f8b35);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeType for WordSegmentsTokenizingHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a5dd6357-bf2a-4c4f-a31f-29e71c6f8b35}");
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
#[doc(hidden)]
pub struct WordSegmentsTokenizingHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precedingwords: *mut ::core::ffi::c_void, words: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invoke: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
