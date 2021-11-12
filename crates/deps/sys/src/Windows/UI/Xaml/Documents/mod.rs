#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Block(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BlockCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Bold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentLinkInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentLinkProviderCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Glyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Hyperlink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HyperlinkClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlock2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlockFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlockStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBlockStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBold(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkProviderCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlyphs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlyphsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlyphsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlink2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlink3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlink4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlink5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInlineFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInlineUIContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItalic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineBreak(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParagraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParagraphStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRun(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRunStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpan(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpanFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElement5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextElementStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHighlighter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHighlighterBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHighlighterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextHighlighterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextPointer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITypography(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITypographyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnderline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Inline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InlineCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InlineUIContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Italic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineBreak(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
#[repr(transparent)]
pub struct Paragraph(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaceContentLinkProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Run(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Span(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextHighlighter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextHighlighterBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextPointer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl ::core::marker::Copy for TextRange {}
impl ::core::clone::Clone for TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Typography(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Underline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
