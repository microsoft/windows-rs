#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct AnnotationType(i32);
#[repr(C)]
pub struct AutomationActiveEnd(i32);
#[repr(C)]
pub struct AutomationAnimationStyle(i32);
#[repr(transparent)]
pub struct AutomationAnnotation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AutomationBulletStyle(i32);
#[repr(C)]
pub struct AutomationCaretBidiMode(i32);
#[repr(C)]
pub struct AutomationCaretPosition(i32);
#[repr(transparent)]
pub struct AutomationElementIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AutomationFlowDirections(i32);
#[repr(C)]
pub struct AutomationOutlineStyles(i32);
#[repr(transparent)]
pub struct AutomationProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationProperty(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AutomationStyleId(i32);
#[repr(C)]
pub struct AutomationTextDecorationLineStyle(i32);
#[repr(C)]
pub struct AutomationTextEditChangeType(i32);
#[repr(transparent)]
pub struct DockPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DockPosition(i32);
#[repr(transparent)]
pub struct DragPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropTargetPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ExpandCollapsePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ExpandCollapseState(i32);
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
#[repr(C)]
pub struct RowOrColumnMajor(i32);
#[repr(C)]
pub struct ScrollAmount(i32);
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
#[repr(C)]
pub struct SupportedTextSelection(i32);
#[repr(C)]
pub struct SynchronizedInputType(i32);
#[repr(transparent)]
pub struct TableItemPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TablePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TogglePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ToggleState(i32);
#[repr(transparent)]
pub struct TransformPattern2Identifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TransformPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ValuePatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WindowInteractionState(i32);
#[repr(transparent)]
pub struct WindowPatternIdentifiers(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WindowVisualState(i32);
#[repr(C)]
pub struct ZoomUnit(i32);
