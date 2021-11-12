#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Xaml_Automation_Peers")]
pub mod Peers;
#[cfg(feature = "UI_Xaml_Automation_Provider")]
pub mod Provider;
#[cfg(feature = "UI_Xaml_Automation_Text")]
pub mod Text;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnnotationPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: Self = Self(60000i32);
    pub const SpellingError: Self = Self(60001i32);
    pub const GrammarError: Self = Self(60002i32);
    pub const Comment: Self = Self(60003i32);
    pub const FormulaError: Self = Self(60004i32);
    pub const TrackChanges: Self = Self(60005i32);
    pub const Header: Self = Self(60006i32);
    pub const Footer: Self = Self(60007i32);
    pub const Highlighted: Self = Self(60008i32);
    pub const Endnote: Self = Self(60009i32);
    pub const Footnote: Self = Self(60010i32);
    pub const InsertionChange: Self = Self(60011i32);
    pub const DeletionChange: Self = Self(60012i32);
    pub const MoveChange: Self = Self(60013i32);
    pub const FormatChange: Self = Self(60014i32);
    pub const UnsyncedChange: Self = Self(60015i32);
    pub const EditingLockedChange: Self = Self(60016i32);
    pub const ExternalChange: Self = Self(60017i32);
    pub const ConflictingChange: Self = Self(60018i32);
    pub const Author: Self = Self(60019i32);
    pub const AdvancedProofingIssue: Self = Self(60020i32);
    pub const DataValidationError: Self = Self(60021i32);
    pub const CircularReferenceError: Self = Self(60022i32);
}
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: Self = Self(0i32);
    pub const LasVegasLights: Self = Self(1i32);
    pub const BlinkingBackground: Self = Self(2i32);
    pub const SparkleText: Self = Self(3i32);
    pub const MarchingBlackAnts: Self = Self(4i32);
    pub const MarchingRedAnts: Self = Self(5i32);
    pub const Shimmer: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
#[repr(transparent)]
pub struct AutomationAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: Self = Self(0i32);
    pub const HollowRoundBullet: Self = Self(1i32);
    pub const FilledRoundBullet: Self = Self(2i32);
    pub const HollowSquareBullet: Self = Self(3i32);
    pub const FilledSquareBullet: Self = Self(4i32);
    pub const DashBullet: Self = Self(5i32);
    pub const Other: Self = Self(6i32);
}
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: Self = Self(0i32);
    pub const RTL: Self = Self(1i32);
}
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: Self = Self(0i32);
    pub const EndOfLine: Self = Self(1i32);
    pub const BeginningOfLine: Self = Self(2i32);
}
#[repr(transparent)]
pub struct AutomationElementIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
    pub const BottomToTop: Self = Self(2i32);
    pub const Vertical: Self = Self(3i32);
}
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: Self = Self(0i32);
    pub const Outline: Self = Self(1i32);
    pub const Shadow: Self = Self(2i32);
    pub const Engraved: Self = Self(3i32);
    pub const Embossed: Self = Self(4i32);
}
#[repr(transparent)]
pub struct AutomationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: Self = Self(70001i32);
    pub const Heading2: Self = Self(70002i32);
    pub const Heading3: Self = Self(70003i32);
    pub const Heading4: Self = Self(70004i32);
    pub const Heading5: Self = Self(70005i32);
    pub const Heading6: Self = Self(70006i32);
    pub const Heading7: Self = Self(70007i32);
    pub const Heading8: Self = Self(70008i32);
    pub const Heading9: Self = Self(70009i32);
    pub const Title: Self = Self(70010i32);
    pub const Subtitle: Self = Self(70011i32);
    pub const Normal: Self = Self(70012i32);
    pub const Emphasis: Self = Self(70013i32);
    pub const Quote: Self = Self(70014i32);
    pub const BulletedList: Self = Self(70015i32);
}
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const WordsOnly: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const Dot: Self = Self(4i32);
    pub const Dash: Self = Self(5i32);
    pub const DashDot: Self = Self(6i32);
    pub const DashDotDot: Self = Self(7i32);
    pub const Wavy: Self = Self(8i32);
    pub const ThickSingle: Self = Self(9i32);
    pub const DoubleWavy: Self = Self(10i32);
    pub const ThickWavy: Self = Self(11i32);
    pub const LongDash: Self = Self(12i32);
    pub const ThickDash: Self = Self(13i32);
    pub const ThickDashDot: Self = Self(14i32);
    pub const ThickDashDotDot: Self = Self(15i32);
    pub const ThickDot: Self = Self(16i32);
    pub const ThickLongDash: Self = Self(17i32);
    pub const Other: Self = Self(18i32);
}
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: Self = Self(0i32);
    pub const AutoCorrect: Self = Self(1i32);
    pub const Composition: Self = Self(2i32);
    pub const CompositionFinalized: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DockPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Fill: Self = Self(4i32);
    pub const None: Self = Self(5i32);
}
#[repr(transparent)]
pub struct DragPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropTargetPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExpandCollapsePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: Self = Self(0i32);
    pub const Expanded: Self = Self(1i32);
    pub const PartiallyExpanded: Self = Self(2i32);
    pub const LeafNode: Self = Self(3i32);
}
#[repr(transparent)]
pub struct GridItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationAnnotationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationAnnotationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationPropertiesStatics9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDockPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDockPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTargetPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTargetPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExpandCollapsePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExpandCollapsePatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridItemPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultipleViewPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultipleViewPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeValuePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRangeValuePatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpreadsheetItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpreadsheetItemPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStylesPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStylesPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableItemPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITablePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITablePatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITogglePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITogglePatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformPattern2Identifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformPattern2IdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransformPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValuePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValuePatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowPatternIdentifiersStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MultipleViewPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RangeValuePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: Self = Self(0i32);
    pub const ColumnMajor: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: Self = Self(0i32);
    pub const SmallDecrement: Self = Self(1i32);
    pub const NoAmount: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
#[repr(transparent)]
pub struct ScrollPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectionItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectionPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpreadsheetItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StylesPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: Self = Self(1i32);
    pub const KeyDown: Self = Self(2i32);
    pub const LeftMouseUp: Self = Self(4i32);
    pub const LeftMouseDown: Self = Self(8i32);
    pub const RightMouseUp: Self = Self(16i32);
    pub const RightMouseDown: Self = Self(32i32);
}
#[repr(transparent)]
pub struct TableItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TablePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TogglePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TransformPattern2Identifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TransformPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ValuePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: Self = Self(0i32);
    pub const Closing: Self = Self(1i32);
    pub const ReadyForUserInteraction: Self = Self(2i32);
    pub const BlockedByModalWindow: Self = Self(3i32);
    pub const NotResponding: Self = Self(4i32);
}
#[repr(transparent)]
pub struct WindowPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: Self = Self(0i32);
    pub const Maximized: Self = Self(1i32);
    pub const Minimized: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: Self = Self(0i32);
    pub const LargeDecrement: Self = Self(1i32);
    pub const SmallDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
