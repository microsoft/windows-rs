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
    pub const Unknown: AnnotationType = AnnotationType(60000i32);
    pub const SpellingError: AnnotationType = AnnotationType(60001i32);
    pub const GrammarError: AnnotationType = AnnotationType(60002i32);
    pub const Comment: AnnotationType = AnnotationType(60003i32);
    pub const FormulaError: AnnotationType = AnnotationType(60004i32);
    pub const TrackChanges: AnnotationType = AnnotationType(60005i32);
    pub const Header: AnnotationType = AnnotationType(60006i32);
    pub const Footer: AnnotationType = AnnotationType(60007i32);
    pub const Highlighted: AnnotationType = AnnotationType(60008i32);
    pub const Endnote: AnnotationType = AnnotationType(60009i32);
    pub const Footnote: AnnotationType = AnnotationType(60010i32);
    pub const InsertionChange: AnnotationType = AnnotationType(60011i32);
    pub const DeletionChange: AnnotationType = AnnotationType(60012i32);
    pub const MoveChange: AnnotationType = AnnotationType(60013i32);
    pub const FormatChange: AnnotationType = AnnotationType(60014i32);
    pub const UnsyncedChange: AnnotationType = AnnotationType(60015i32);
    pub const EditingLockedChange: AnnotationType = AnnotationType(60016i32);
    pub const ExternalChange: AnnotationType = AnnotationType(60017i32);
    pub const ConflictingChange: AnnotationType = AnnotationType(60018i32);
    pub const Author: AnnotationType = AnnotationType(60019i32);
    pub const AdvancedProofingIssue: AnnotationType = AnnotationType(60020i32);
    pub const DataValidationError: AnnotationType = AnnotationType(60021i32);
    pub const CircularReferenceError: AnnotationType = AnnotationType(60022i32);
}
#[repr(transparent)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: AutomationActiveEnd = AutomationActiveEnd(0i32);
    pub const Start: AutomationActiveEnd = AutomationActiveEnd(1i32);
    pub const End: AutomationActiveEnd = AutomationActiveEnd(2i32);
}
#[repr(transparent)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: AutomationAnimationStyle = AutomationAnimationStyle(0i32);
    pub const LasVegasLights: AutomationAnimationStyle = AutomationAnimationStyle(1i32);
    pub const BlinkingBackground: AutomationAnimationStyle = AutomationAnimationStyle(2i32);
    pub const SparkleText: AutomationAnimationStyle = AutomationAnimationStyle(3i32);
    pub const MarchingBlackAnts: AutomationAnimationStyle = AutomationAnimationStyle(4i32);
    pub const MarchingRedAnts: AutomationAnimationStyle = AutomationAnimationStyle(5i32);
    pub const Shimmer: AutomationAnimationStyle = AutomationAnimationStyle(6i32);
    pub const Other: AutomationAnimationStyle = AutomationAnimationStyle(7i32);
}
#[repr(transparent)]
pub struct AutomationAnnotation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: AutomationBulletStyle = AutomationBulletStyle(0i32);
    pub const HollowRoundBullet: AutomationBulletStyle = AutomationBulletStyle(1i32);
    pub const FilledRoundBullet: AutomationBulletStyle = AutomationBulletStyle(2i32);
    pub const HollowSquareBullet: AutomationBulletStyle = AutomationBulletStyle(3i32);
    pub const FilledSquareBullet: AutomationBulletStyle = AutomationBulletStyle(4i32);
    pub const DashBullet: AutomationBulletStyle = AutomationBulletStyle(5i32);
    pub const Other: AutomationBulletStyle = AutomationBulletStyle(6i32);
}
#[repr(transparent)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: AutomationCaretBidiMode = AutomationCaretBidiMode(0i32);
    pub const RTL: AutomationCaretBidiMode = AutomationCaretBidiMode(1i32);
}
#[repr(transparent)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: AutomationCaretPosition = AutomationCaretPosition(0i32);
    pub const EndOfLine: AutomationCaretPosition = AutomationCaretPosition(1i32);
    pub const BeginningOfLine: AutomationCaretPosition = AutomationCaretPosition(2i32);
}
#[repr(transparent)]
pub struct AutomationElementIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: AutomationFlowDirections = AutomationFlowDirections(0i32);
    pub const RightToLeft: AutomationFlowDirections = AutomationFlowDirections(1i32);
    pub const BottomToTop: AutomationFlowDirections = AutomationFlowDirections(2i32);
    pub const Vertical: AutomationFlowDirections = AutomationFlowDirections(3i32);
}
#[repr(transparent)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: AutomationOutlineStyles = AutomationOutlineStyles(0i32);
    pub const Outline: AutomationOutlineStyles = AutomationOutlineStyles(1i32);
    pub const Shadow: AutomationOutlineStyles = AutomationOutlineStyles(2i32);
    pub const Engraved: AutomationOutlineStyles = AutomationOutlineStyles(3i32);
    pub const Embossed: AutomationOutlineStyles = AutomationOutlineStyles(4i32);
}
#[repr(transparent)]
pub struct AutomationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: AutomationStyleId = AutomationStyleId(70001i32);
    pub const Heading2: AutomationStyleId = AutomationStyleId(70002i32);
    pub const Heading3: AutomationStyleId = AutomationStyleId(70003i32);
    pub const Heading4: AutomationStyleId = AutomationStyleId(70004i32);
    pub const Heading5: AutomationStyleId = AutomationStyleId(70005i32);
    pub const Heading6: AutomationStyleId = AutomationStyleId(70006i32);
    pub const Heading7: AutomationStyleId = AutomationStyleId(70007i32);
    pub const Heading8: AutomationStyleId = AutomationStyleId(70008i32);
    pub const Heading9: AutomationStyleId = AutomationStyleId(70009i32);
    pub const Title: AutomationStyleId = AutomationStyleId(70010i32);
    pub const Subtitle: AutomationStyleId = AutomationStyleId(70011i32);
    pub const Normal: AutomationStyleId = AutomationStyleId(70012i32);
    pub const Emphasis: AutomationStyleId = AutomationStyleId(70013i32);
    pub const Quote: AutomationStyleId = AutomationStyleId(70014i32);
    pub const BulletedList: AutomationStyleId = AutomationStyleId(70015i32);
}
#[repr(transparent)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(0i32);
    pub const Single: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(1i32);
    pub const WordsOnly: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(2i32);
    pub const Double: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(3i32);
    pub const Dot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(4i32);
    pub const Dash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(5i32);
    pub const DashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(6i32);
    pub const DashDotDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(7i32);
    pub const Wavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(8i32);
    pub const ThickSingle: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(9i32);
    pub const DoubleWavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(10i32);
    pub const ThickWavy: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(11i32);
    pub const LongDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(12i32);
    pub const ThickDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(13i32);
    pub const ThickDashDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(14i32);
    pub const ThickDashDotDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(15i32);
    pub const ThickDot: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(16i32);
    pub const ThickLongDash: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(17i32);
    pub const Other: AutomationTextDecorationLineStyle = AutomationTextDecorationLineStyle(18i32);
}
#[repr(transparent)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: AutomationTextEditChangeType = AutomationTextEditChangeType(0i32);
    pub const AutoCorrect: AutomationTextEditChangeType = AutomationTextEditChangeType(1i32);
    pub const Composition: AutomationTextEditChangeType = AutomationTextEditChangeType(2i32);
    pub const CompositionFinalized: AutomationTextEditChangeType = AutomationTextEditChangeType(3i32);
}
#[repr(transparent)]
pub struct DockPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: DockPosition = DockPosition(0i32);
    pub const Left: DockPosition = DockPosition(1i32);
    pub const Bottom: DockPosition = DockPosition(2i32);
    pub const Right: DockPosition = DockPosition(3i32);
    pub const Fill: DockPosition = DockPosition(4i32);
    pub const None: DockPosition = DockPosition(5i32);
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
    pub const Collapsed: ExpandCollapseState = ExpandCollapseState(0i32);
    pub const Expanded: ExpandCollapseState = ExpandCollapseState(1i32);
    pub const PartiallyExpanded: ExpandCollapseState = ExpandCollapseState(2i32);
    pub const LeafNode: ExpandCollapseState = ExpandCollapseState(3i32);
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
    pub const RowMajor: RowOrColumnMajor = RowOrColumnMajor(0i32);
    pub const ColumnMajor: RowOrColumnMajor = RowOrColumnMajor(1i32);
    pub const Indeterminate: RowOrColumnMajor = RowOrColumnMajor(2i32);
}
#[repr(transparent)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: ScrollAmount = ScrollAmount(0i32);
    pub const SmallDecrement: ScrollAmount = ScrollAmount(1i32);
    pub const NoAmount: ScrollAmount = ScrollAmount(2i32);
    pub const LargeIncrement: ScrollAmount = ScrollAmount(3i32);
    pub const SmallIncrement: ScrollAmount = ScrollAmount(4i32);
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
    pub const None: SupportedTextSelection = SupportedTextSelection(0i32);
    pub const Single: SupportedTextSelection = SupportedTextSelection(1i32);
    pub const Multiple: SupportedTextSelection = SupportedTextSelection(2i32);
}
#[repr(transparent)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: SynchronizedInputType = SynchronizedInputType(1i32);
    pub const KeyDown: SynchronizedInputType = SynchronizedInputType(2i32);
    pub const LeftMouseUp: SynchronizedInputType = SynchronizedInputType(4i32);
    pub const LeftMouseDown: SynchronizedInputType = SynchronizedInputType(8i32);
    pub const RightMouseUp: SynchronizedInputType = SynchronizedInputType(16i32);
    pub const RightMouseDown: SynchronizedInputType = SynchronizedInputType(32i32);
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
    pub const Off: ToggleState = ToggleState(0i32);
    pub const On: ToggleState = ToggleState(1i32);
    pub const Indeterminate: ToggleState = ToggleState(2i32);
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
    pub const Running: WindowInteractionState = WindowInteractionState(0i32);
    pub const Closing: WindowInteractionState = WindowInteractionState(1i32);
    pub const ReadyForUserInteraction: WindowInteractionState = WindowInteractionState(2i32);
    pub const BlockedByModalWindow: WindowInteractionState = WindowInteractionState(3i32);
    pub const NotResponding: WindowInteractionState = WindowInteractionState(4i32);
}
#[repr(transparent)]
pub struct WindowPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: WindowVisualState = WindowVisualState(0i32);
    pub const Maximized: WindowVisualState = WindowVisualState(1i32);
    pub const Minimized: WindowVisualState = WindowVisualState(2i32);
}
#[repr(transparent)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: ZoomUnit = ZoomUnit(0i32);
    pub const LargeDecrement: ZoomUnit = ZoomUnit(1i32);
    pub const SmallDecrement: ZoomUnit = ZoomUnit(2i32);
    pub const LargeIncrement: ZoomUnit = ZoomUnit(3i32);
    pub const SmallIncrement: ZoomUnit = ZoomUnit(4i32);
}
