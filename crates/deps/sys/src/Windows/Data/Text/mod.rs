#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AlternateNormalizationFormat(pub i32);
impl AlternateNormalizationFormat {
    pub const NotNormalized: AlternateNormalizationFormat = AlternateNormalizationFormat(0i32);
    pub const Number: AlternateNormalizationFormat = AlternateNormalizationFormat(1i32);
    pub const Currency: AlternateNormalizationFormat = AlternateNormalizationFormat(3i32);
    pub const Date: AlternateNormalizationFormat = AlternateNormalizationFormat(4i32);
    pub const Time: AlternateNormalizationFormat = AlternateNormalizationFormat(5i32);
}
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
#[repr(transparent)]
pub struct TextPredictionOptions(pub u32);
impl TextPredictionOptions {
    pub const None: TextPredictionOptions = TextPredictionOptions(0u32);
    pub const Predictions: TextPredictionOptions = TextPredictionOptions(1u32);
    pub const Corrections: TextPredictionOptions = TextPredictionOptions(2u32);
}
#[repr(transparent)]
pub struct TextReverseConversionGenerator(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TextSegment(i32);
#[repr(transparent)]
pub struct UnicodeGeneralCategory(pub i32);
impl UnicodeGeneralCategory {
    pub const UppercaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(0i32);
    pub const LowercaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(1i32);
    pub const TitlecaseLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(2i32);
    pub const ModifierLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(3i32);
    pub const OtherLetter: UnicodeGeneralCategory = UnicodeGeneralCategory(4i32);
    pub const NonspacingMark: UnicodeGeneralCategory = UnicodeGeneralCategory(5i32);
    pub const SpacingCombiningMark: UnicodeGeneralCategory = UnicodeGeneralCategory(6i32);
    pub const EnclosingMark: UnicodeGeneralCategory = UnicodeGeneralCategory(7i32);
    pub const DecimalDigitNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(8i32);
    pub const LetterNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(9i32);
    pub const OtherNumber: UnicodeGeneralCategory = UnicodeGeneralCategory(10i32);
    pub const SpaceSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(11i32);
    pub const LineSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(12i32);
    pub const ParagraphSeparator: UnicodeGeneralCategory = UnicodeGeneralCategory(13i32);
    pub const Control: UnicodeGeneralCategory = UnicodeGeneralCategory(14i32);
    pub const Format: UnicodeGeneralCategory = UnicodeGeneralCategory(15i32);
    pub const Surrogate: UnicodeGeneralCategory = UnicodeGeneralCategory(16i32);
    pub const PrivateUse: UnicodeGeneralCategory = UnicodeGeneralCategory(17i32);
    pub const ConnectorPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(18i32);
    pub const DashPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(19i32);
    pub const OpenPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(20i32);
    pub const ClosePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(21i32);
    pub const InitialQuotePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(22i32);
    pub const FinalQuotePunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(23i32);
    pub const OtherPunctuation: UnicodeGeneralCategory = UnicodeGeneralCategory(24i32);
    pub const MathSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(25i32);
    pub const CurrencySymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(26i32);
    pub const ModifierSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(27i32);
    pub const OtherSymbol: UnicodeGeneralCategory = UnicodeGeneralCategory(28i32);
    pub const NotAssigned: UnicodeGeneralCategory = UnicodeGeneralCategory(29i32);
}
#[repr(transparent)]
pub struct UnicodeNumericType(pub i32);
impl UnicodeNumericType {
    pub const None: UnicodeNumericType = UnicodeNumericType(0i32);
    pub const Decimal: UnicodeNumericType = UnicodeNumericType(1i32);
    pub const Digit: UnicodeNumericType = UnicodeNumericType(2i32);
    pub const Numeric: UnicodeNumericType = UnicodeNumericType(3i32);
}
#[repr(transparent)]
pub struct WordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordSegmentsTokenizingHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordsSegmenter(pub *mut ::core::ffi::c_void);
