#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAnnotationPatternIdentifiersStaticsImpl: Sized {
    fn AnnotationTypeIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationTypeNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AuthorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DateTimeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn TargetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<AnnotationType>;
    fn SetType(&self, value: AnnotationType) -> ::windows::core::Result<()>;
    fn Element(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetElement(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationFactoryImpl: Sized {
    fn CreateInstance(&self, r#type: AnnotationType) -> ::windows::core::Result<AutomationAnnotation>;
    fn CreateWithElementParameter(&self, r#type: AnnotationType, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<AutomationAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationAnnotationStaticsImpl: Sized {
    fn TypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ElementProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStaticsImpl: Sized {
    fn AcceleratorKeyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AutomationIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn BoundingRectangleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ClassNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ClickablePointProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ControlTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HasKeyboardFocusProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HelpTextProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsContentElementProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsControlElementProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsEnabledProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsKeyboardFocusableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsOffscreenProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsPasswordProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsRequiredForFormProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemStatusProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ItemTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LabeledByProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedControlTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn NameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn OrientationProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LiveSettingProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics2Impl: Sized {
    fn ControlledPeersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics3Impl: Sized {
    fn PositionInSetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SizeOfSetProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn AnnotationsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics4Impl: Sized {
    fn LandmarkTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LocalizedLandmarkTypeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics5Impl: Sized {
    fn IsPeripheralProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsDataValidForFormProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FullDescriptionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DescribedByProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsToProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FlowsFromProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics6Impl: Sized {
    fn CultureProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics7Impl: Sized {
    fn HeadingLevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementIdentifiersStatics8Impl: Sized {
    fn IsDialogProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStaticsImpl: Sized {
    fn AcceleratorKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAcceleratorKey(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcceleratorKey(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessKey(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AutomationIdProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationId(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAutomationId(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HelpTextProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHelpText(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHelpText(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsRequiredForFormProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsRequiredForForm(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsRequiredForForm(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn ItemStatusProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemStatus(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemStatus(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetItemType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetItemType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LabeledByProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLabeledBy(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::UIElement>;
    fn SetLabeledBy(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn NameProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetName(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LiveSettingProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLiveSetting(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLiveSetting>;
    fn SetLiveSetting(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLiveSetting) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics2Impl: Sized {
    fn AccessibilityViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAccessibilityView(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AccessibilityView>;
    fn SetAccessibilityView(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AccessibilityView) -> ::windows::core::Result<()>;
    fn ControlledPeersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetControlledPeers(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics3Impl: Sized {
    fn PositionInSetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetPositionInSet(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetPositionInSet(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn SizeOfSetProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetSizeOfSet(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetSizeOfSet(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn LevelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLevel(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetLevel(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
    fn AnnotationsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAnnotations(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics4Impl: Sized {
    fn LandmarkTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationLandmarkType>;
    fn SetLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationLandmarkType) -> ::windows::core::Result<()>;
    fn LocalizedLandmarkTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedLandmarkType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics5Impl: Sized {
    fn IsPeripheralProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsPeripheral(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsPeripheral(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn IsDataValidForFormProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDataValidForForm(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDataValidForForm(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
    fn FullDescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFullDescription(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFullDescription(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LocalizedControlTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetLocalizedControlType(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalizedControlType(&self, element: &::core::option::Option<super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DescribedByProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDescribedBy(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsToProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsTo(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
    fn FlowsFromProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetFlowsFrom(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics6Impl: Sized {
    fn CultureProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetCulture(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<i32>;
    fn SetCulture(&self, element: &::core::option::Option<super::DependencyObject>, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics7Impl: Sized {
    fn HeadingLevelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetHeadingLevel(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<Peers::AutomationHeadingLevel>;
    fn SetHeadingLevel(&self, element: &::core::option::Option<super::DependencyObject>, value: Peers::AutomationHeadingLevel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics8Impl: Sized {
    fn IsDialogProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetIsDialog(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
    fn SetIsDialog(&self, element: &::core::option::Option<super::DependencyObject>, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertiesStatics9Impl: Sized {
    fn AutomationControlTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetAutomationControlType(&self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<Peers::AutomationControlType>;
    fn SetAutomationControlType(&self, element: &::core::option::Option<super::UIElement>, value: Peers::AutomationControlType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPropertyImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDockPatternIdentifiersStaticsImpl: Sized {
    fn DockPositionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDragPatternIdentifiersStaticsImpl: Sized {
    fn DropEffectProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DropEffectsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn GrabbedItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsGrabbedProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDropTargetPatternIdentifiersStaticsImpl: Sized {
    fn DropTargetEffectProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn DropTargetEffectsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IExpandCollapsePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IExpandCollapsePatternIdentifiersStaticsImpl: Sized {
    fn ExpandCollapseStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridItemPatternIdentifiersStaticsImpl: Sized {
    fn ColumnProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ColumnSpanProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ContainingGridProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowSpanProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IGridPatternIdentifiersStaticsImpl: Sized {
    fn ColumnCountProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowCountProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultipleViewPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMultipleViewPatternIdentifiersStaticsImpl: Sized {
    fn CurrentViewProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SupportedViewsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeValuePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeValuePatternIdentifiersStaticsImpl: Sized {
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn LargeChangeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MaximumProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MinimumProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SmallChangeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollPatternIdentifiersStaticsImpl: Sized {
    fn HorizontallyScrollableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalScrollPercentProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn HorizontalViewSizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn NoScroll(&self) -> ::windows::core::Result<f64>;
    fn VerticallyScrollableProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalScrollPercentProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn VerticalViewSizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionItemPatternIdentifiersStaticsImpl: Sized {
    fn IsSelectedProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionContainerProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectionPatternIdentifiersStaticsImpl: Sized {
    fn CanSelectMultipleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsSelectionRequiredProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn SelectionProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpreadsheetItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ISpreadsheetItemPatternIdentifiersStaticsImpl: Sized {
    fn FormulaProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStylesPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IStylesPatternIdentifiersStaticsImpl: Sized {
    fn ExtendedPropertiesProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillColorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternColorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn FillPatternStyleProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ShapeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleIdProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn StyleNameProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITableItemPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITableItemPatternIdentifiersStaticsImpl: Sized {
    fn ColumnHeaderItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeaderItemsProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITablePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITablePatternIdentifiersStaticsImpl: Sized {
    fn ColumnHeadersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowHeadersProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn RowOrColumnMajorProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITogglePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITogglePatternIdentifiersStaticsImpl: Sized {
    fn ToggleStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPattern2IdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPattern2IdentifiersStaticsImpl: Sized {
    fn CanZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ZoomLevelProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MaxZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn MinZoomProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransformPatternIdentifiersStaticsImpl: Sized {
    fn CanMoveProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanResizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanRotateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IValuePatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IValuePatternIdentifiersStaticsImpl: Sized {
    fn IsReadOnlyProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowPatternIdentifiersImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowPatternIdentifiersStaticsImpl: Sized {
    fn CanMaximizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn CanMinimizeProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsModalProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn IsTopmostProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowInteractionStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
    fn WindowVisualStateProperty(&self) -> ::windows::core::Result<AutomationProperty>;
}
