#[cfg(feature = "implement_exclusive")]
pub trait IContentLinkInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, value: u32) -> ::windows::core::Result<()>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SecondaryText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSecondaryText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LinkContentKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLinkContentKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFontWeightsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IFontWeightsStaticsImpl: Sized {
    fn Black(&self) -> ::windows::core::Result<FontWeight>;
    fn Bold(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBlack(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraBold(&self) -> ::windows::core::Result<FontWeight>;
    fn ExtraLight(&self) -> ::windows::core::Result<FontWeight>;
    fn Light(&self) -> ::windows::core::Result<FontWeight>;
    fn Medium(&self) -> ::windows::core::Result<FontWeight>;
    fn Normal(&self) -> ::windows::core::Result<FontWeight>;
    fn SemiBold(&self) -> ::windows::core::Result<FontWeight>;
    fn SemiLight(&self) -> ::windows::core::Result<FontWeight>;
    fn Thin(&self) -> ::windows::core::Result<FontWeight>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditTextRangeImpl: Sized {
    fn ContentLinkInfo(&self) -> ::windows::core::Result<ContentLinkInfo>;
    fn SetContentLinkInfo(&self, value: &::core::option::Option<ContentLinkInfo>) -> ::windows::core::Result<()>;
}
pub trait ITextCharacterFormatImpl: Sized {
    fn AllCaps(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetAllCaps(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetBackgroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Bold(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetBold(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn FontStretch(&self) -> ::windows::core::Result<FontStretch>;
    fn SetFontStretch(&self, value: FontStretch) -> ::windows::core::Result<()>;
    fn FontStyle(&self) -> ::windows::core::Result<FontStyle>;
    fn SetFontStyle(&self, value: FontStyle) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::Color>;
    fn SetForegroundColor(&self, value: &super::Color) -> ::windows::core::Result<()>;
    fn Hidden(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetHidden(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Italic(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetItalic(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Kerning(&self) -> ::windows::core::Result<f32>;
    fn SetKerning(&self, value: f32) -> ::windows::core::Result<()>;
    fn LanguageTag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguageTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LinkType(&self) -> ::windows::core::Result<LinkType>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Outline(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetOutline(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<f32>;
    fn SetPosition(&self, value: f32) -> ::windows::core::Result<()>;
    fn ProtectedText(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetProtectedText(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<f32>;
    fn SetSize(&self, value: f32) -> ::windows::core::Result<()>;
    fn SmallCaps(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSmallCaps(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Spacing(&self) -> ::windows::core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> ::windows::core::Result<()>;
    fn Strikethrough(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetStrikethrough(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Subscript(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSubscript(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Superscript(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetSuperscript(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TextScript(&self) -> ::windows::core::Result<TextScript>;
    fn SetTextScript(&self, value: TextScript) -> ::windows::core::Result<()>;
    fn Underline(&self) -> ::windows::core::Result<UnderlineType>;
    fn SetUnderline(&self, value: UnderlineType) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<i32>;
    fn SetWeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn SetClone(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn IsEqual(&self, format: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextConstantsStaticsImpl: Sized {
    fn AutoColor(&self) -> ::windows::core::Result<super::Color>;
    fn MinUnitCount(&self) -> ::windows::core::Result<i32>;
    fn MaxUnitCount(&self) -> ::windows::core::Result<i32>;
    fn UndefinedColor(&self) -> ::windows::core::Result<super::Color>;
    fn UndefinedFloatValue(&self) -> ::windows::core::Result<f32>;
    fn UndefinedInt32Value(&self) -> ::windows::core::Result<i32>;
    fn UndefinedFontStretch(&self) -> ::windows::core::Result<FontStretch>;
    fn UndefinedFontStyle(&self) -> ::windows::core::Result<FontStyle>;
}
pub trait ITextDocumentImpl: Sized {
    fn CaretType(&self) -> ::windows::core::Result<CaretType>;
    fn SetCaretType(&self, value: CaretType) -> ::windows::core::Result<()>;
    fn DefaultTabStop(&self) -> ::windows::core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> ::windows::core::Result<()>;
    fn Selection(&self) -> ::windows::core::Result<ITextSelection>;
    fn UndoLimit(&self) -> ::windows::core::Result<u32>;
    fn SetUndoLimit(&self, value: u32) -> ::windows::core::Result<()>;
    fn CanCopy(&self) -> ::windows::core::Result<bool>;
    fn CanPaste(&self) -> ::windows::core::Result<bool>;
    fn CanRedo(&self) -> ::windows::core::Result<bool>;
    fn CanUndo(&self) -> ::windows::core::Result<bool>;
    fn ApplyDisplayUpdates(&self) -> ::windows::core::Result<i32>;
    fn BatchDisplayUpdates(&self) -> ::windows::core::Result<i32>;
    fn BeginUndoGroup(&self) -> ::windows::core::Result<()>;
    fn EndUndoGroup(&self) -> ::windows::core::Result<()>;
    fn GetDefaultCharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<ITextRange>;
    fn GetRangeFromPoint(&self, point: &super::super::Foundation::Point, options: PointOptions) -> ::windows::core::Result<ITextRange>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadFromStream(&self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn Redo(&self) -> ::windows::core::Result<()>;
    fn SaveToStream(&self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn SetDefaultCharacterFormat(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn SetDefaultParagraphFormat(&self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Undo(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument2Impl: Sized {
    fn AlignmentIncludesTrailingWhitespace(&self) -> ::windows::core::Result<bool>;
    fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> ::windows::core::Result<()>;
    fn IgnoreTrailingCharacterSpacing(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument3Impl: Sized {
    fn ClearUndoRedoHistory(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextDocument4Impl: Sized {
    fn SetMath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetMath(&self, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetMathMode(&self, mode: RichEditMathMode) -> ::windows::core::Result<()>;
}
pub trait ITextParagraphFormatImpl: Sized {
    fn Alignment(&self) -> ::windows::core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows::core::Result<()>;
    fn FirstLineIndent(&self) -> ::windows::core::Result<f32>;
    fn KeepTogether(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn KeepWithNext(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn LeftIndent(&self) -> ::windows::core::Result<f32>;
    fn LineSpacing(&self) -> ::windows::core::Result<f32>;
    fn LineSpacingRule(&self) -> ::windows::core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> ::windows::core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows::core::Result<()>;
    fn ListLevelIndex(&self) -> ::windows::core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn ListStart(&self) -> ::windows::core::Result<i32>;
    fn SetListStart(&self, value: i32) -> ::windows::core::Result<()>;
    fn ListStyle(&self) -> ::windows::core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> ::windows::core::Result<()>;
    fn ListTab(&self) -> ::windows::core::Result<f32>;
    fn SetListTab(&self, value: f32) -> ::windows::core::Result<()>;
    fn ListType(&self) -> ::windows::core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> ::windows::core::Result<()>;
    fn NoLineNumber(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn PageBreakBefore(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn RightIndent(&self) -> ::windows::core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> ::windows::core::Result<()>;
    fn RightToLeft(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn Style(&self) -> ::windows::core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> ::windows::core::Result<()>;
    fn SpaceAfter(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> ::windows::core::Result<()>;
    fn SpaceBefore(&self) -> ::windows::core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> ::windows::core::Result<()>;
    fn WidowControl(&self) -> ::windows::core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> ::windows::core::Result<()>;
    fn TabCount(&self) -> ::windows::core::Result<i32>;
    fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::core::Result<()>;
    fn ClearAllTabs(&self) -> ::windows::core::Result<()>;
    fn DeleteTab(&self, position: f32) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows::core::Result<()>;
    fn IsEqual(&self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<bool>;
    fn SetClone(&self, format: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows::core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> ::windows::core::Result<()>;
}
pub trait ITextRangeImpl: Sized {
    fn Character(&self) -> ::windows::core::Result<u16>;
    fn SetCharacter(&self, value: u16) -> ::windows::core::Result<()>;
    fn CharacterFormat(&self) -> ::windows::core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&self, value: &::core::option::Option<ITextCharacterFormat>) -> ::windows::core::Result<()>;
    fn FormattedText(&self) -> ::windows::core::Result<ITextRange>;
    fn SetFormattedText(&self, value: &::core::option::Option<ITextRange>) -> ::windows::core::Result<()>;
    fn EndPosition(&self) -> ::windows::core::Result<i32>;
    fn SetEndPosition(&self, value: i32) -> ::windows::core::Result<()>;
    fn Gravity(&self) -> ::windows::core::Result<RangeGravity>;
    fn SetGravity(&self, value: RangeGravity) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn Link(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLink(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ParagraphFormat(&self) -> ::windows::core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&self, value: &::core::option::Option<ITextParagraphFormat>) -> ::windows::core::Result<()>;
    fn StartPosition(&self) -> ::windows::core::Result<i32>;
    fn SetStartPosition(&self, value: i32) -> ::windows::core::Result<()>;
    fn StoryLength(&self) -> ::windows::core::Result<i32>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CanPaste(&self, format: i32) -> ::windows::core::Result<bool>;
    fn ChangeCase(&self, value: LetterCase) -> ::windows::core::Result<()>;
    fn Collapse(&self, value: bool) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<()>;
    fn Cut(&self) -> ::windows::core::Result<()>;
    fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn Expand(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn FindText(&self, value: &::windows::core::HSTRING, scanlength: i32, options: FindOptions) -> ::windows::core::Result<i32>;
    fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::core::Result<()>;
    fn GetClone(&self) -> ::windows::core::Result<ITextRange>;
    fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::core::Result<i32>;
    fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::core::Result<()>;
    fn GetText(&self, options: TextGetOptions, value: &mut ::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetTextViaStream(&self, options: TextGetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InRange(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &::windows::core::HSTRING, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn InStory(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn IsEqual(&self, range: &::core::option::Option<ITextRange>) -> ::windows::core::Result<bool>;
    fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::core::Result<i32>;
    fn Paste(&self, format: i32) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, value: PointOptions) -> ::windows::core::Result<()>;
    fn MatchSelection(&self) -> ::windows::core::Result<()>;
    fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::core::Result<()>;
    fn SetPoint(&self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::core::Result<()>;
    fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetTextViaStream(&self, options: TextSetOptions, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<()>;
    fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
}
pub trait ITextSelectionImpl: Sized + ITextRangeImpl {
    fn Options(&self) -> ::windows::core::Result<SelectionOptions>;
    fn SetOptions(&self, value: SelectionOptions) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<SelectionType>;
    fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::core::Result<i32>;
    fn TypeText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
