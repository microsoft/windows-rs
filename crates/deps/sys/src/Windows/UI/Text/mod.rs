#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct CaretType(i32);
#[repr(transparent)]
pub struct ContentLinkInfo(pub *mut ::core::ffi::c_void);
pub struct FindOptions(i32);
pub struct FontStretch(i32);
pub struct FontStyle(i32);
pub struct FontWeight(i32);
#[repr(transparent)]
pub struct FontWeights(pub *mut ::core::ffi::c_void);
pub struct FormatEffect(i32);
pub struct HorizontalCharacterAlignment(i32);
#[repr(transparent)]
pub struct IContentLinkInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontWeights(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontWeightsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditTextRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCharacterFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextConstantsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextDocument4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextParagraphFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextSelection(pub *mut ::core::ffi::c_void);
pub struct LetterCase(i32);
pub struct LineSpacingRule(i32);
pub struct LinkType(i32);
pub struct MarkerAlignment(i32);
pub struct MarkerStyle(i32);
pub struct MarkerType(i32);
pub struct ParagraphAlignment(i32);
pub struct ParagraphStyle(i32);
pub struct PointOptions(i32);
pub struct RangeGravity(i32);
pub struct RichEditMathMode(i32);
#[repr(transparent)]
pub struct RichEditTextDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditTextRange(pub *mut ::core::ffi::c_void);
pub struct SelectionOptions(i32);
pub struct SelectionType(i32);
pub struct TabAlignment(i32);
pub struct TabLeader(i32);
#[repr(transparent)]
pub struct TextConstants(pub *mut ::core::ffi::c_void);
pub struct TextDecorations(i32);
pub struct TextGetOptions(i32);
pub struct TextRangeUnit(i32);
pub struct TextScript(i32);
pub struct TextSetOptions(i32);
pub struct UnderlineType(i32);
pub struct VerticalCharacterAlignment(i32);
