#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CaretType(i32);
#[repr(transparent)]
pub struct ContentLinkInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FindOptions(i32);
#[repr(C)]
pub struct FontStretch(i32);
#[repr(C)]
pub struct FontStyle(i32);
#[repr(C)]
pub struct FontWeight(i32);
#[repr(transparent)]
pub struct FontWeights(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FormatEffect(i32);
#[repr(C)]
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
#[repr(C)]
pub struct LetterCase(i32);
#[repr(C)]
pub struct LineSpacingRule(i32);
#[repr(C)]
pub struct LinkType(i32);
#[repr(C)]
pub struct MarkerAlignment(i32);
#[repr(C)]
pub struct MarkerStyle(i32);
#[repr(C)]
pub struct MarkerType(i32);
#[repr(C)]
pub struct ParagraphAlignment(i32);
#[repr(C)]
pub struct ParagraphStyle(i32);
#[repr(C)]
pub struct PointOptions(i32);
#[repr(C)]
pub struct RangeGravity(i32);
#[repr(C)]
pub struct RichEditMathMode(i32);
#[repr(transparent)]
pub struct RichEditTextDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditTextRange(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SelectionOptions(i32);
#[repr(C)]
pub struct SelectionType(i32);
#[repr(C)]
pub struct TabAlignment(i32);
#[repr(C)]
pub struct TabLeader(i32);
#[repr(C)]
pub struct TextDecorations(i32);
#[repr(C)]
pub struct TextGetOptions(i32);
#[repr(C)]
pub struct TextRangeUnit(i32);
#[repr(C)]
pub struct TextScript(i32);
#[repr(C)]
pub struct TextSetOptions(i32);
#[repr(C)]
pub struct UnderlineType(i32);
#[repr(C)]
pub struct VerticalCharacterAlignment(i32);
