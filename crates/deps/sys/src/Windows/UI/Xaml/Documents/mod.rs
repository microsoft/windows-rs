#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Block(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Block {}
impl ::core::clone::Clone for Block {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BlockCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BlockCollection {}
impl ::core::clone::Clone for BlockCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Bold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Bold {}
impl ::core::clone::Clone for Bold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactContentLinkProvider {}
impl ::core::clone::Clone for ContactContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentLink {}
impl ::core::clone::Clone for ContentLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentLinkInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentLinkInvokedEventArgs {}
impl ::core::clone::Clone for ContentLinkInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentLinkProvider {}
impl ::core::clone::Clone for ContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentLinkProviderCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentLinkProviderCollection {}
impl ::core::clone::Clone for ContentLinkProviderCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Glyphs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Glyphs {}
impl ::core::clone::Clone for Glyphs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Hyperlink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Hyperlink {}
impl ::core::clone::Clone for Hyperlink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HyperlinkClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HyperlinkClickEventArgs {}
impl ::core::clone::Clone for HyperlinkClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlock {}
impl ::core::clone::Clone for IBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlock2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlock2 {}
impl ::core::clone::Clone for IBlock2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlockFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockFactory {}
impl ::core::clone::Clone for IBlockFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlockStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockStatics {}
impl ::core::clone::Clone for IBlockStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBlockStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBlockStatics2 {}
impl ::core::clone::Clone for IBlockStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBold(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBold {}
impl ::core::clone::Clone for IBold {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactContentLinkProvider {}
impl ::core::clone::Clone for IContactContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLink {}
impl ::core::clone::Clone for IContentLink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkInvokedEventArgs {}
impl ::core::clone::Clone for IContentLinkInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkProvider {}
impl ::core::clone::Clone for IContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkProviderCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkProviderCollection {}
impl ::core::clone::Clone for IContentLinkProviderCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkProviderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkProviderFactory {}
impl ::core::clone::Clone for IContentLinkProviderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkStatics {}
impl ::core::clone::Clone for IContentLinkStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlyphs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlyphs {}
impl ::core::clone::Clone for IGlyphs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlyphs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlyphs2 {}
impl ::core::clone::Clone for IGlyphs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlyphsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlyphsStatics {}
impl ::core::clone::Clone for IGlyphsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlyphsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlyphsStatics2 {}
impl ::core::clone::Clone for IGlyphsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlink {}
impl ::core::clone::Clone for IHyperlink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlink2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlink2 {}
impl ::core::clone::Clone for IHyperlink2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlink3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlink3 {}
impl ::core::clone::Clone for IHyperlink3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlink4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlink4 {}
impl ::core::clone::Clone for IHyperlink4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlink5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlink5 {}
impl ::core::clone::Clone for IHyperlink5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkClickEventArgs {}
impl ::core::clone::Clone for IHyperlinkClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkStatics {}
impl ::core::clone::Clone for IHyperlinkStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkStatics2 {}
impl ::core::clone::Clone for IHyperlinkStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkStatics3 {}
impl ::core::clone::Clone for IHyperlinkStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkStatics4 {}
impl ::core::clone::Clone for IHyperlinkStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkStatics5 {}
impl ::core::clone::Clone for IHyperlinkStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInline {}
impl ::core::clone::Clone for IInline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInlineFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInlineFactory {}
impl ::core::clone::Clone for IInlineFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInlineUIContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInlineUIContainer {}
impl ::core::clone::Clone for IInlineUIContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItalic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItalic {}
impl ::core::clone::Clone for IItalic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineBreak(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineBreak {}
impl ::core::clone::Clone for ILineBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IParagraph(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IParagraph {}
impl ::core::clone::Clone for IParagraph {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IParagraphStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IParagraphStatics {}
impl ::core::clone::Clone for IParagraphStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaceContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaceContentLinkProvider {}
impl ::core::clone::Clone for IPlaceContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRun(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRun {}
impl ::core::clone::Clone for IRun {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRunStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRunStatics {}
impl ::core::clone::Clone for IRunStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpan(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpan {}
impl ::core::clone::Clone for ISpan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpanFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpanFactory {}
impl ::core::clone::Clone for ISpanFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElement {}
impl ::core::clone::Clone for ITextElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElement2 {}
impl ::core::clone::Clone for ITextElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElement3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElement3 {}
impl ::core::clone::Clone for ITextElement3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElement4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElement4 {}
impl ::core::clone::Clone for ITextElement4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElement5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElement5 {}
impl ::core::clone::Clone for ITextElement5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementFactory {}
impl ::core::clone::Clone for ITextElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementOverrides {}
impl ::core::clone::Clone for ITextElementOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementStatics {}
impl ::core::clone::Clone for ITextElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementStatics2 {}
impl ::core::clone::Clone for ITextElementStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementStatics3 {}
impl ::core::clone::Clone for ITextElementStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextElementStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextElementStatics4 {}
impl ::core::clone::Clone for ITextElementStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHighlighter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHighlighter {}
impl ::core::clone::Clone for ITextHighlighter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHighlighterBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHighlighterBase {}
impl ::core::clone::Clone for ITextHighlighterBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHighlighterBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHighlighterBaseFactory {}
impl ::core::clone::Clone for ITextHighlighterBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHighlighterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHighlighterFactory {}
impl ::core::clone::Clone for ITextHighlighterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextHighlighterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextHighlighterStatics {}
impl ::core::clone::Clone for ITextHighlighterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextPointer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextPointer {}
impl ::core::clone::Clone for ITextPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypography(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypography {}
impl ::core::clone::Clone for ITypography {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITypographyStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITypographyStatics {}
impl ::core::clone::Clone for ITypographyStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnderline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnderline {}
impl ::core::clone::Clone for IUnderline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Inline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Inline {}
impl ::core::clone::Clone for Inline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InlineCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InlineCollection {}
impl ::core::clone::Clone for InlineCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InlineUIContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InlineUIContainer {}
impl ::core::clone::Clone for InlineUIContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Italic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Italic {}
impl ::core::clone::Clone for Italic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineBreak(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineBreak {}
impl ::core::clone::Clone for LineBreak {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LogicalDirection(pub i32);
impl LogicalDirection {
    pub const Backward: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
}
impl ::core::marker::Copy for LogicalDirection {}
impl ::core::clone::Clone for LogicalDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Paragraph(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Paragraph {}
impl ::core::clone::Clone for Paragraph {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaceContentLinkProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaceContentLinkProvider {}
impl ::core::clone::Clone for PlaceContentLinkProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Run(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Run {}
impl ::core::clone::Clone for Run {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Span(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Span {}
impl ::core::clone::Clone for Span {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextElement {}
impl ::core::clone::Clone for TextElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextHighlighter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextHighlighter {}
impl ::core::clone::Clone for TextHighlighter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextHighlighterBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextHighlighterBase {}
impl ::core::clone::Clone for TextHighlighterBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextPointer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextPointer {}
impl ::core::clone::Clone for TextPointer {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for Typography {}
impl ::core::clone::Clone for Typography {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Underline(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Underline {}
impl ::core::clone::Clone for Underline {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnderlineStyle(pub i32);
impl UnderlineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
}
impl ::core::marker::Copy for UnderlineStyle {}
impl ::core::clone::Clone for UnderlineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
