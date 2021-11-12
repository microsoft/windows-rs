#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AlternateNormalizationFormat(i32);
#[repr(transparent)]
pub struct AlternateWordForm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlternateWordForm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectableWordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectableWordsSegmenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectableWordsSegmenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticTextQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticTextQueryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextConversionGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextConversionGeneratorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPhoneme(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPredictionGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPredictionGenerator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPredictionGeneratorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextReverseConversionGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextReverseConversionGenerator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextReverseConversionGeneratorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnicodeCharactersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordsSegmenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWordsSegmenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectableWordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectableWordSegmentsTokenizingHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectableWordsSegmenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SemanticTextQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextConversionGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextPhoneme(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextPredictionGenerator(pub *mut ::core::ffi::c_void);
pub struct TextPredictionOptions(i32);
#[repr(transparent)]
pub struct TextReverseConversionGenerator(pub *mut ::core::ffi::c_void);
pub struct TextSegment(i32);
#[repr(transparent)]
pub struct UnicodeCharacters(pub *mut ::core::ffi::c_void);
pub struct UnicodeGeneralCategory(i32);
pub struct UnicodeNumericType(i32);
#[repr(transparent)]
pub struct WordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordSegmentsTokenizingHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordsSegmenter(pub *mut ::core::ffi::c_void);
