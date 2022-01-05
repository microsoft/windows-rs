pub trait IAnnotationProviderImpl: Sized {
    fn AnnotationTypeId(&self) -> ::windows::core::Result<i32>;
    fn AnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DateTime(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Target(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
}
pub trait ICustomNavigationProviderImpl: Sized {
    fn NavigateCustom(&self, direction: super::Peers::AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait IDockProviderImpl: Sized {
    fn DockPosition(&self) -> ::windows::core::Result<super::DockPosition>;
    fn SetDockPosition(&self, dockposition: super::DockPosition) -> ::windows::core::Result<()>;
}
pub trait IDragProviderImpl: Sized {
    fn IsGrabbed(&self) -> ::windows::core::Result<bool>;
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
    fn GetGrabbedItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
pub trait IDropTargetProviderImpl: Sized {
    fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
pub trait IExpandCollapseProviderImpl: Sized {
    fn ExpandCollapseState(&self) -> ::windows::core::Result<super::ExpandCollapseState>;
    fn Collapse(&self) -> ::windows::core::Result<()>;
    fn Expand(&self) -> ::windows::core::Result<()>;
}
pub trait IGridItemProviderImpl: Sized {
    fn Column(&self) -> ::windows::core::Result<i32>;
    fn ColumnSpan(&self) -> ::windows::core::Result<i32>;
    fn ContainingGrid(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn Row(&self) -> ::windows::core::Result<i32>;
    fn RowSpan(&self) -> ::windows::core::Result<i32>;
}
pub trait IGridProviderImpl: Sized {
    fn ColumnCount(&self) -> ::windows::core::Result<i32>;
    fn RowCount(&self) -> ::windows::core::Result<i32>;
    fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIRawElementProviderSimpleImpl: Sized {}
pub trait IInvokeProviderImpl: Sized {
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty(&self, startafter: &::core::option::Option<IRawElementProviderSimple>, automationproperty: &::core::option::Option<super::AutomationProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
pub trait IMultipleViewProviderImpl: Sized {
    fn CurrentView(&self) -> ::windows::core::Result<i32>;
    fn GetSupportedViews(&self) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn GetViewName(&self, viewid: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurrentView(&self, viewid: i32) -> ::windows::core::Result<()>;
}
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait IRangeValueProviderImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn LargeChange(&self) -> ::windows::core::Result<f64>;
    fn Maximum(&self) -> ::windows::core::Result<f64>;
    fn Minimum(&self) -> ::windows::core::Result<f64>;
    fn SmallChange(&self) -> ::windows::core::Result<f64>;
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView(&self) -> ::windows::core::Result<()>;
}
pub trait IScrollProviderImpl: Sized {
    fn HorizontallyScrollable(&self) -> ::windows::core::Result<bool>;
    fn HorizontalScrollPercent(&self) -> ::windows::core::Result<f64>;
    fn HorizontalViewSize(&self) -> ::windows::core::Result<f64>;
    fn VerticallyScrollable(&self) -> ::windows::core::Result<bool>;
    fn VerticalScrollPercent(&self) -> ::windows::core::Result<f64>;
    fn VerticalViewSize(&self) -> ::windows::core::Result<f64>;
    fn Scroll(&self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()>;
}
pub trait ISelectionItemProviderImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SelectionContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn AddToSelection(&self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows::core::Result<()>;
    fn Select(&self) -> ::windows::core::Result<()>;
}
pub trait ISelectionProviderImpl: Sized {
    fn CanSelectMultiple(&self) -> ::windows::core::Result<bool>;
    fn IsSelectionRequired(&self) -> ::windows::core::Result<bool>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
pub trait ISpreadsheetItemProviderImpl: Sized {
    fn Formula(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAnnotationObjects(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetAnnotationTypes(&self) -> ::windows::core::Result<::windows::core::Array<super::AnnotationType>>;
}
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IRawElementProviderSimple>;
}
pub trait IStylesProviderImpl: Sized {
    fn ExtendedProperties(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn FillPatternColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn FillPatternStyle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Shape(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StyleId(&self) -> ::windows::core::Result<i32>;
    fn StyleName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait ISynchronizedInputProviderImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn StartListening(&self, inputtype: super::SynchronizedInputType) -> ::windows::core::Result<()>;
}
pub trait ITableItemProviderImpl: Sized {
    fn GetColumnHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
pub trait ITableProviderImpl: Sized {
    fn RowOrColumnMajor(&self) -> ::windows::core::Result<super::RowOrColumnMajor>;
    fn GetColumnHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
    fn GetRowHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
pub trait ITextChildProviderImpl: Sized {
    fn TextContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn TextRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider>;
}
pub trait ITextProviderImpl: Sized {
    fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection>;
    fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>>;
    fn RangeFromChild(&self, childelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, screenlocation: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<ITextRangeProvider>;
}
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation(&self, annotationelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: &mut bool) -> ::windows::core::Result<ITextRangeProvider>;
}
pub trait ITextRangeProviderImpl: Sized {
    fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider>;
    fn Compare(&self, textrangeprovider: &::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<bool>;
    fn CompareEndpoints(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: &::core::option::Option<ITextRangeProvider>, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&self, attributeid: i32, value: &::core::option::Option<::windows::core::IInspectable>, backward: bool) -> ::windows::core::Result<ITextRangeProvider>;
    fn FindText(&self, text: &::windows::core::HSTRING, backward: bool, ignorecase: bool) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&self, attributeid: i32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetBoundingRectangles(&self, returnvalue: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()>;
    fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: &::core::option::Option<ITextRangeProvider>, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&self) -> ::windows::core::Result<()>;
    fn AddToSelection(&self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::core::Result<()>;
    fn GetChildren(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>>;
}
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
}
pub trait IToggleProviderImpl: Sized {
    fn ToggleState(&self) -> ::windows::core::Result<super::ToggleState>;
    fn Toggle(&self) -> ::windows::core::Result<()>;
}
pub trait ITransformProviderImpl: Sized {
    fn CanMove(&self) -> ::windows::core::Result<bool>;
    fn CanResize(&self) -> ::windows::core::Result<bool>;
    fn CanRotate(&self) -> ::windows::core::Result<bool>;
    fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()>;
    fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()>;
}
pub trait ITransformProvider2Impl: Sized + ITransformProviderImpl {
    fn CanZoom(&self) -> ::windows::core::Result<bool>;
    fn ZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn MaxZoom(&self) -> ::windows::core::Result<f64>;
    fn MinZoom(&self) -> ::windows::core::Result<f64>;
    fn Zoom(&self, zoom: f64) -> ::windows::core::Result<()>;
    fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> ::windows::core::Result<()>;
}
pub trait IValueProviderImpl: Sized {
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize(&self) -> ::windows::core::Result<()>;
}
pub trait IWindowProviderImpl: Sized {
    fn IsModal(&self) -> ::windows::core::Result<bool>;
    fn IsTopmost(&self) -> ::windows::core::Result<bool>;
    fn Maximizable(&self) -> ::windows::core::Result<bool>;
    fn Minimizable(&self) -> ::windows::core::Result<bool>;
    fn InteractionState(&self) -> ::windows::core::Result<super::WindowInteractionState>;
    fn VisualState(&self) -> ::windows::core::Result<super::WindowVisualState>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn SetVisualState(&self, state: super::WindowVisualState) -> ::windows::core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::core::Result<bool>;
}
