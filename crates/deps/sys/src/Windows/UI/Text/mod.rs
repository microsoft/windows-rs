#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn CaretType();
    fn ContentLinkInfo();
    fn FindOptions();
    fn FontStretch();
    fn FontStyle();
    fn FontWeight();
    fn FontWeights();
    fn FormatEffect();
    fn HorizontalCharacterAlignment();
    fn IContentLinkInfo();
    fn IFontWeights();
    fn IFontWeightsStatics();
    fn IRichEditTextRange();
    fn ITextCharacterFormat();
    fn ITextConstantsStatics();
    fn ITextDocument();
    fn ITextDocument2();
    fn ITextDocument3();
    fn ITextDocument4();
    fn ITextParagraphFormat();
    fn ITextRange();
    fn ITextSelection();
    fn LetterCase();
    fn LineSpacingRule();
    fn LinkType();
    fn MarkerAlignment();
    fn MarkerStyle();
    fn MarkerType();
    fn ParagraphAlignment();
    fn ParagraphStyle();
    fn PointOptions();
    fn RangeGravity();
    fn RichEditMathMode();
    fn RichEditTextDocument();
    fn RichEditTextRange();
    fn SelectionOptions();
    fn SelectionType();
    fn TabAlignment();
    fn TabLeader();
    fn TextConstants();
    fn TextDecorations();
    fn TextGetOptions();
    fn TextRangeUnit();
    fn TextScript();
    fn TextSetOptions();
    fn UnderlineType();
    fn VerticalCharacterAlignment();
}
