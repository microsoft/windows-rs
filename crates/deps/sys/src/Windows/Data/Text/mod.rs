#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
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
#[repr(transparent)]
pub struct TextReverseConversionGenerator(pub *mut ::core::ffi::c_void);
#[repr(C)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct WordSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordSegmentsTokenizingHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WordsSegmenter(pub *mut ::core::ffi::c_void);
