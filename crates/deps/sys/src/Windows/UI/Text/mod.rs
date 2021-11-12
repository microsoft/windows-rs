#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: CaretType = CaretType(0i32);
    pub const Null: CaretType = CaretType(1i32);
}
#[repr(transparent)]
pub struct ContentLinkInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: FindOptions = FindOptions(0u32);
    pub const Word: FindOptions = FindOptions(2u32);
    pub const Case: FindOptions = FindOptions(4u32);
}
#[repr(transparent)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: FontStretch = FontStretch(0i32);
    pub const UltraCondensed: FontStretch = FontStretch(1i32);
    pub const ExtraCondensed: FontStretch = FontStretch(2i32);
    pub const Condensed: FontStretch = FontStretch(3i32);
    pub const SemiCondensed: FontStretch = FontStretch(4i32);
    pub const Normal: FontStretch = FontStretch(5i32);
    pub const SemiExpanded: FontStretch = FontStretch(6i32);
    pub const Expanded: FontStretch = FontStretch(7i32);
    pub const ExtraExpanded: FontStretch = FontStretch(8i32);
    pub const UltraExpanded: FontStretch = FontStretch(9i32);
}
#[repr(transparent)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: FontStyle = FontStyle(0i32);
    pub const Oblique: FontStyle = FontStyle(1i32);
    pub const Italic: FontStyle = FontStyle(2i32);
}
#[repr(C)]
pub struct FontWeight(i32);
#[repr(transparent)]
pub struct FontWeights(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: FormatEffect = FormatEffect(0i32);
    pub const On: FormatEffect = FormatEffect(1i32);
    pub const Toggle: FormatEffect = FormatEffect(2i32);
    pub const Undefined: FormatEffect = FormatEffect(3i32);
}
#[repr(transparent)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: HorizontalCharacterAlignment = HorizontalCharacterAlignment(0i32);
    pub const Right: HorizontalCharacterAlignment = HorizontalCharacterAlignment(1i32);
    pub const Center: HorizontalCharacterAlignment = HorizontalCharacterAlignment(2i32);
}
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
#[repr(transparent)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: LetterCase = LetterCase(0i32);
    pub const Upper: LetterCase = LetterCase(1i32);
}
#[repr(transparent)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: LineSpacingRule = LineSpacingRule(0i32);
    pub const Single: LineSpacingRule = LineSpacingRule(1i32);
    pub const OneAndHalf: LineSpacingRule = LineSpacingRule(2i32);
    pub const Double: LineSpacingRule = LineSpacingRule(3i32);
    pub const AtLeast: LineSpacingRule = LineSpacingRule(4i32);
    pub const Exactly: LineSpacingRule = LineSpacingRule(5i32);
    pub const Multiple: LineSpacingRule = LineSpacingRule(6i32);
    pub const Percent: LineSpacingRule = LineSpacingRule(7i32);
}
#[repr(transparent)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: LinkType = LinkType(0i32);
    pub const NotALink: LinkType = LinkType(1i32);
    pub const ClientLink: LinkType = LinkType(2i32);
    pub const FriendlyLinkName: LinkType = LinkType(3i32);
    pub const FriendlyLinkAddress: LinkType = LinkType(4i32);
    pub const AutoLink: LinkType = LinkType(5i32);
    pub const AutoLinkEmail: LinkType = LinkType(6i32);
    pub const AutoLinkPhone: LinkType = LinkType(7i32);
    pub const AutoLinkPath: LinkType = LinkType(8i32);
}
#[repr(transparent)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: MarkerAlignment = MarkerAlignment(0i32);
    pub const Left: MarkerAlignment = MarkerAlignment(1i32);
    pub const Center: MarkerAlignment = MarkerAlignment(2i32);
    pub const Right: MarkerAlignment = MarkerAlignment(3i32);
}
#[repr(transparent)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: MarkerStyle = MarkerStyle(0i32);
    pub const Parenthesis: MarkerStyle = MarkerStyle(1i32);
    pub const Parentheses: MarkerStyle = MarkerStyle(2i32);
    pub const Period: MarkerStyle = MarkerStyle(3i32);
    pub const Plain: MarkerStyle = MarkerStyle(4i32);
    pub const Minus: MarkerStyle = MarkerStyle(5i32);
    pub const NoNumber: MarkerStyle = MarkerStyle(6i32);
}
#[repr(transparent)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: MarkerType = MarkerType(0i32);
    pub const None: MarkerType = MarkerType(1i32);
    pub const Bullet: MarkerType = MarkerType(2i32);
    pub const Arabic: MarkerType = MarkerType(3i32);
    pub const LowercaseEnglishLetter: MarkerType = MarkerType(4i32);
    pub const UppercaseEnglishLetter: MarkerType = MarkerType(5i32);
    pub const LowercaseRoman: MarkerType = MarkerType(6i32);
    pub const UppercaseRoman: MarkerType = MarkerType(7i32);
    pub const UnicodeSequence: MarkerType = MarkerType(8i32);
    pub const CircledNumber: MarkerType = MarkerType(9i32);
    pub const BlackCircleWingding: MarkerType = MarkerType(10i32);
    pub const WhiteCircleWingding: MarkerType = MarkerType(11i32);
    pub const ArabicWide: MarkerType = MarkerType(12i32);
    pub const SimplifiedChinese: MarkerType = MarkerType(13i32);
    pub const TraditionalChinese: MarkerType = MarkerType(14i32);
    pub const JapanSimplifiedChinese: MarkerType = MarkerType(15i32);
    pub const JapanKorea: MarkerType = MarkerType(16i32);
    pub const ArabicDictionary: MarkerType = MarkerType(17i32);
    pub const ArabicAbjad: MarkerType = MarkerType(18i32);
    pub const Hebrew: MarkerType = MarkerType(19i32);
    pub const ThaiAlphabetic: MarkerType = MarkerType(20i32);
    pub const ThaiNumeric: MarkerType = MarkerType(21i32);
    pub const DevanagariVowel: MarkerType = MarkerType(22i32);
    pub const DevanagariConsonant: MarkerType = MarkerType(23i32);
    pub const DevanagariNumeric: MarkerType = MarkerType(24i32);
}
#[repr(transparent)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: ParagraphAlignment = ParagraphAlignment(0i32);
    pub const Left: ParagraphAlignment = ParagraphAlignment(1i32);
    pub const Center: ParagraphAlignment = ParagraphAlignment(2i32);
    pub const Right: ParagraphAlignment = ParagraphAlignment(3i32);
    pub const Justify: ParagraphAlignment = ParagraphAlignment(4i32);
}
#[repr(transparent)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: ParagraphStyle = ParagraphStyle(0i32);
    pub const None: ParagraphStyle = ParagraphStyle(1i32);
    pub const Normal: ParagraphStyle = ParagraphStyle(2i32);
    pub const Heading1: ParagraphStyle = ParagraphStyle(3i32);
    pub const Heading2: ParagraphStyle = ParagraphStyle(4i32);
    pub const Heading3: ParagraphStyle = ParagraphStyle(5i32);
    pub const Heading4: ParagraphStyle = ParagraphStyle(6i32);
    pub const Heading5: ParagraphStyle = ParagraphStyle(7i32);
    pub const Heading6: ParagraphStyle = ParagraphStyle(8i32);
    pub const Heading7: ParagraphStyle = ParagraphStyle(9i32);
    pub const Heading8: ParagraphStyle = ParagraphStyle(10i32);
    pub const Heading9: ParagraphStyle = ParagraphStyle(11i32);
}
#[repr(transparent)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: PointOptions = PointOptions(0u32);
    pub const IncludeInset: PointOptions = PointOptions(1u32);
    pub const Start: PointOptions = PointOptions(32u32);
    pub const ClientCoordinates: PointOptions = PointOptions(256u32);
    pub const AllowOffClient: PointOptions = PointOptions(512u32);
    pub const Transform: PointOptions = PointOptions(1024u32);
    pub const NoHorizontalScroll: PointOptions = PointOptions(65536u32);
    pub const NoVerticalScroll: PointOptions = PointOptions(262144u32);
}
#[repr(transparent)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: RangeGravity = RangeGravity(0i32);
    pub const Backward: RangeGravity = RangeGravity(1i32);
    pub const Forward: RangeGravity = RangeGravity(2i32);
    pub const Inward: RangeGravity = RangeGravity(3i32);
    pub const Outward: RangeGravity = RangeGravity(4i32);
}
#[repr(transparent)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: RichEditMathMode = RichEditMathMode(0i32);
    pub const MathOnly: RichEditMathMode = RichEditMathMode(1i32);
}
#[repr(transparent)]
pub struct RichEditTextDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditTextRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: SelectionOptions = SelectionOptions(1u32);
    pub const AtEndOfLine: SelectionOptions = SelectionOptions(2u32);
    pub const Overtype: SelectionOptions = SelectionOptions(4u32);
    pub const Active: SelectionOptions = SelectionOptions(8u32);
    pub const Replace: SelectionOptions = SelectionOptions(16u32);
}
#[repr(transparent)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: SelectionType = SelectionType(0i32);
    pub const InsertionPoint: SelectionType = SelectionType(1i32);
    pub const Normal: SelectionType = SelectionType(2i32);
    pub const InlineShape: SelectionType = SelectionType(7i32);
    pub const Shape: SelectionType = SelectionType(8i32);
}
#[repr(transparent)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: TabAlignment = TabAlignment(0i32);
    pub const Center: TabAlignment = TabAlignment(1i32);
    pub const Right: TabAlignment = TabAlignment(2i32);
    pub const Decimal: TabAlignment = TabAlignment(3i32);
    pub const Bar: TabAlignment = TabAlignment(4i32);
}
#[repr(transparent)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: TabLeader = TabLeader(0i32);
    pub const Dots: TabLeader = TabLeader(1i32);
    pub const Dashes: TabLeader = TabLeader(2i32);
    pub const Lines: TabLeader = TabLeader(3i32);
    pub const ThickLines: TabLeader = TabLeader(4i32);
    pub const Equals: TabLeader = TabLeader(5i32);
}
#[repr(transparent)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: TextDecorations = TextDecorations(0u32);
    pub const Underline: TextDecorations = TextDecorations(1u32);
    pub const Strikethrough: TextDecorations = TextDecorations(2u32);
}
#[repr(transparent)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: TextGetOptions = TextGetOptions(0u32);
    pub const AdjustCrlf: TextGetOptions = TextGetOptions(1u32);
    pub const UseCrlf: TextGetOptions = TextGetOptions(2u32);
    pub const UseObjectText: TextGetOptions = TextGetOptions(4u32);
    pub const AllowFinalEop: TextGetOptions = TextGetOptions(8u32);
    pub const NoHidden: TextGetOptions = TextGetOptions(32u32);
    pub const IncludeNumbering: TextGetOptions = TextGetOptions(64u32);
    pub const FormatRtf: TextGetOptions = TextGetOptions(8192u32);
    pub const UseLf: TextGetOptions = TextGetOptions(16777216u32);
}
#[repr(transparent)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: TextRangeUnit = TextRangeUnit(0i32);
    pub const Word: TextRangeUnit = TextRangeUnit(1i32);
    pub const Sentence: TextRangeUnit = TextRangeUnit(2i32);
    pub const Paragraph: TextRangeUnit = TextRangeUnit(3i32);
    pub const Line: TextRangeUnit = TextRangeUnit(4i32);
    pub const Story: TextRangeUnit = TextRangeUnit(5i32);
    pub const Screen: TextRangeUnit = TextRangeUnit(6i32);
    pub const Section: TextRangeUnit = TextRangeUnit(7i32);
    pub const Window: TextRangeUnit = TextRangeUnit(8i32);
    pub const CharacterFormat: TextRangeUnit = TextRangeUnit(9i32);
    pub const ParagraphFormat: TextRangeUnit = TextRangeUnit(10i32);
    pub const Object: TextRangeUnit = TextRangeUnit(11i32);
    pub const HardParagraph: TextRangeUnit = TextRangeUnit(12i32);
    pub const Cluster: TextRangeUnit = TextRangeUnit(13i32);
    pub const Bold: TextRangeUnit = TextRangeUnit(14i32);
    pub const Italic: TextRangeUnit = TextRangeUnit(15i32);
    pub const Underline: TextRangeUnit = TextRangeUnit(16i32);
    pub const Strikethrough: TextRangeUnit = TextRangeUnit(17i32);
    pub const ProtectedText: TextRangeUnit = TextRangeUnit(18i32);
    pub const Link: TextRangeUnit = TextRangeUnit(19i32);
    pub const SmallCaps: TextRangeUnit = TextRangeUnit(20i32);
    pub const AllCaps: TextRangeUnit = TextRangeUnit(21i32);
    pub const Hidden: TextRangeUnit = TextRangeUnit(22i32);
    pub const Outline: TextRangeUnit = TextRangeUnit(23i32);
    pub const Shadow: TextRangeUnit = TextRangeUnit(24i32);
    pub const Imprint: TextRangeUnit = TextRangeUnit(25i32);
    pub const Disabled: TextRangeUnit = TextRangeUnit(26i32);
    pub const Revised: TextRangeUnit = TextRangeUnit(27i32);
    pub const Subscript: TextRangeUnit = TextRangeUnit(28i32);
    pub const Superscript: TextRangeUnit = TextRangeUnit(29i32);
    pub const FontBound: TextRangeUnit = TextRangeUnit(30i32);
    pub const LinkProtected: TextRangeUnit = TextRangeUnit(31i32);
    pub const ContentLink: TextRangeUnit = TextRangeUnit(32i32);
}
#[repr(transparent)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: TextScript = TextScript(0i32);
    pub const Ansi: TextScript = TextScript(1i32);
    pub const EastEurope: TextScript = TextScript(2i32);
    pub const Cyrillic: TextScript = TextScript(3i32);
    pub const Greek: TextScript = TextScript(4i32);
    pub const Turkish: TextScript = TextScript(5i32);
    pub const Hebrew: TextScript = TextScript(6i32);
    pub const Arabic: TextScript = TextScript(7i32);
    pub const Baltic: TextScript = TextScript(8i32);
    pub const Vietnamese: TextScript = TextScript(9i32);
    pub const Default: TextScript = TextScript(10i32);
    pub const Symbol: TextScript = TextScript(11i32);
    pub const Thai: TextScript = TextScript(12i32);
    pub const ShiftJis: TextScript = TextScript(13i32);
    pub const GB2312: TextScript = TextScript(14i32);
    pub const Hangul: TextScript = TextScript(15i32);
    pub const Big5: TextScript = TextScript(16i32);
    pub const PC437: TextScript = TextScript(17i32);
    pub const Oem: TextScript = TextScript(18i32);
    pub const Mac: TextScript = TextScript(19i32);
    pub const Armenian: TextScript = TextScript(20i32);
    pub const Syriac: TextScript = TextScript(21i32);
    pub const Thaana: TextScript = TextScript(22i32);
    pub const Devanagari: TextScript = TextScript(23i32);
    pub const Bengali: TextScript = TextScript(24i32);
    pub const Gurmukhi: TextScript = TextScript(25i32);
    pub const Gujarati: TextScript = TextScript(26i32);
    pub const Oriya: TextScript = TextScript(27i32);
    pub const Tamil: TextScript = TextScript(28i32);
    pub const Telugu: TextScript = TextScript(29i32);
    pub const Kannada: TextScript = TextScript(30i32);
    pub const Malayalam: TextScript = TextScript(31i32);
    pub const Sinhala: TextScript = TextScript(32i32);
    pub const Lao: TextScript = TextScript(33i32);
    pub const Tibetan: TextScript = TextScript(34i32);
    pub const Myanmar: TextScript = TextScript(35i32);
    pub const Georgian: TextScript = TextScript(36i32);
    pub const Jamo: TextScript = TextScript(37i32);
    pub const Ethiopic: TextScript = TextScript(38i32);
    pub const Cherokee: TextScript = TextScript(39i32);
    pub const Aboriginal: TextScript = TextScript(40i32);
    pub const Ogham: TextScript = TextScript(41i32);
    pub const Runic: TextScript = TextScript(42i32);
    pub const Khmer: TextScript = TextScript(43i32);
    pub const Mongolian: TextScript = TextScript(44i32);
    pub const Braille: TextScript = TextScript(45i32);
    pub const Yi: TextScript = TextScript(46i32);
    pub const Limbu: TextScript = TextScript(47i32);
    pub const TaiLe: TextScript = TextScript(48i32);
    pub const NewTaiLue: TextScript = TextScript(49i32);
    pub const SylotiNagri: TextScript = TextScript(50i32);
    pub const Kharoshthi: TextScript = TextScript(51i32);
    pub const Kayahli: TextScript = TextScript(52i32);
    pub const UnicodeSymbol: TextScript = TextScript(53i32);
    pub const Emoji: TextScript = TextScript(54i32);
    pub const Glagolitic: TextScript = TextScript(55i32);
    pub const Lisu: TextScript = TextScript(56i32);
    pub const Vai: TextScript = TextScript(57i32);
    pub const NKo: TextScript = TextScript(58i32);
    pub const Osmanya: TextScript = TextScript(59i32);
    pub const PhagsPa: TextScript = TextScript(60i32);
    pub const Gothic: TextScript = TextScript(61i32);
    pub const Deseret: TextScript = TextScript(62i32);
    pub const Tifinagh: TextScript = TextScript(63i32);
}
#[repr(transparent)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: TextSetOptions = TextSetOptions(0u32);
    pub const UnicodeBidi: TextSetOptions = TextSetOptions(1u32);
    pub const Unlink: TextSetOptions = TextSetOptions(8u32);
    pub const Unhide: TextSetOptions = TextSetOptions(16u32);
    pub const CheckTextLimit: TextSetOptions = TextSetOptions(32u32);
    pub const FormatRtf: TextSetOptions = TextSetOptions(8192u32);
    pub const ApplyRtfDocumentDefaults: TextSetOptions = TextSetOptions(16384u32);
}
#[repr(transparent)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: UnderlineType = UnderlineType(0i32);
    pub const None: UnderlineType = UnderlineType(1i32);
    pub const Single: UnderlineType = UnderlineType(2i32);
    pub const Words: UnderlineType = UnderlineType(3i32);
    pub const Double: UnderlineType = UnderlineType(4i32);
    pub const Dotted: UnderlineType = UnderlineType(5i32);
    pub const Dash: UnderlineType = UnderlineType(6i32);
    pub const DashDot: UnderlineType = UnderlineType(7i32);
    pub const DashDotDot: UnderlineType = UnderlineType(8i32);
    pub const Wave: UnderlineType = UnderlineType(9i32);
    pub const Thick: UnderlineType = UnderlineType(10i32);
    pub const Thin: UnderlineType = UnderlineType(11i32);
    pub const DoubleWave: UnderlineType = UnderlineType(12i32);
    pub const HeavyWave: UnderlineType = UnderlineType(13i32);
    pub const LongDash: UnderlineType = UnderlineType(14i32);
    pub const ThickDash: UnderlineType = UnderlineType(15i32);
    pub const ThickDashDot: UnderlineType = UnderlineType(16i32);
    pub const ThickDashDotDot: UnderlineType = UnderlineType(17i32);
    pub const ThickDotted: UnderlineType = UnderlineType(18i32);
    pub const ThickLongDash: UnderlineType = UnderlineType(19i32);
}
#[repr(transparent)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: VerticalCharacterAlignment = VerticalCharacterAlignment(0i32);
    pub const Baseline: VerticalCharacterAlignment = VerticalCharacterAlignment(1i32);
    pub const Bottom: VerticalCharacterAlignment = VerticalCharacterAlignment(2i32);
}
