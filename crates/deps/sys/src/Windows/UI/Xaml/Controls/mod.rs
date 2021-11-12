#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_Xaml_Controls_Maps")]
pub mod Maps;
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
pub mod Primitives;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnchorRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarButton(pub *mut ::core::ffi::c_void);
pub struct AppBarClosedDisplayMode(i32);
#[repr(transparent)]
pub struct AppBarElementContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBarToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoSuggestBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoSuggestBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoSuggestBoxSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutoSuggestBoxTextChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct AutoSuggestionBoxTextChangeReason(i32);
#[repr(transparent)]
pub struct BackClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackClickEventHandler(pub *mut ::core::ffi::c_void);
pub struct BackgroundSizing(i32);
#[repr(transparent)]
pub struct BitmapIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BitmapIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Border(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Button(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarDatePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarDatePickerDateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarViewDayItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarViewDayItemChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarViewDayItemChangingEventHandler(pub *mut ::core::ffi::c_void);
pub struct CalendarViewDisplayMode(i32);
#[repr(transparent)]
pub struct CalendarViewSelectedDatesChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct CalendarViewSelectionMode(i32);
pub struct CandidateWindowAlignment(i32);
#[repr(transparent)]
pub struct CandidateWindowBoundsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Canvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CaptureElement(pub *mut ::core::ffi::c_void);
pub struct CharacterCasing(i32);
#[repr(transparent)]
pub struct CheckBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChoosingGroupHeaderContainerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChoosingItemContainerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CleanUpVirtualizedItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CleanUpVirtualizedItemEventHandler(pub *mut ::core::ffi::c_void);
pub struct ClickMode(i32);
#[repr(transparent)]
pub struct ColorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPicker(pub *mut ::core::ffi::c_void);
pub struct ColorPickerHsvChannel(i32);
pub struct ColorSpectrumComponents(i32);
pub struct ColorSpectrumShape(i32);
#[repr(transparent)]
pub struct ColumnDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColumnDefinitionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxItem(pub *mut ::core::ffi::c_void);
pub struct ComboBoxSelectionChangedTrigger(i32);
#[repr(transparent)]
pub struct ComboBoxTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBar(pub *mut ::core::ffi::c_void);
pub struct CommandBarDefaultLabelPosition(i32);
pub struct CommandBarDynamicOverflowAction(i32);
#[repr(transparent)]
pub struct CommandBarFlyout(pub *mut ::core::ffi::c_void);
pub struct CommandBarLabelPosition(i32);
pub struct CommandBarOverflowButtonVisibility(i32);
#[repr(transparent)]
pub struct CommandBarOverflowPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContainerContentChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialog(pub *mut ::core::ffi::c_void);
pub struct ContentDialogButton(i32);
#[repr(transparent)]
pub struct ContentDialogButtonClickDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogButtonClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogClosingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogOpenedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ContentDialogPlacement(i32);
pub struct ContentDialogResult(i32);
pub struct ContentLinkChangeKind(i32);
#[repr(transparent)]
pub struct ContentLinkChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContextMenuEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContextMenuOpeningEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Control(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ControlTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataTemplateSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DatePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct DisabledFormattingAccelerators(i32);
#[repr(transparent)]
pub struct DragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DragItemsStartingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropDownButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DropDownButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DynamicOverflowItemsChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlipView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlipViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Flyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusDisengagedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FocusEngagedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FontIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FontIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Frame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Grid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GridViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GroupItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GroupStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GroupStyleSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandwritingPanelClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandwritingPanelOpenedEventArgs(pub *mut ::core::ffi::c_void);
pub struct HandwritingPanelPlacementAlignment(i32);
#[repr(transparent)]
pub struct HandwritingView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandwritingViewCandidatesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HandwritingViewTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Hub(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubSectionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubSectionHeaderClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HubSectionHeaderClickEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HyperlinkButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnchorRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBar2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBar3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBar4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButton3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButton4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButton5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarButtonStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarElementContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarElementContainerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarElementContainerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarOverrides3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarSeparatorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarSeparatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarSeparatorStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButton3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButton4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButton5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxTextChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoSuggestBoxTextChangedEventArgsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIcon2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBitmapIconStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBorder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBorder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBorderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBorderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonStaticsWithFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IButtonWithFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerDateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarDatePickerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewDayItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewDayItemChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewDayItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewDayItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewSelectedDatesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICalendarViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICandidateWindowBoundsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICanvasStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICaptureElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICheckBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICheckBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChoosingGroupHeaderContainerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChoosingItemContainerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICleanUpVirtualizedItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorPickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnDefinitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBox6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComboBoxTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBar2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBar3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarOverflowPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarOverflowPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommandBarStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContainerContentChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentControlOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialog2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialog3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogButtonClickDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogButtonClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogClosingDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentDialogStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentLinkChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenter4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenter5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPresenterStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControl7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlOverrides6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IControlTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateSelector2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateSelectorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateSelectorOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTemplateSelectorOverrides2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyout2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerFlyoutStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDatePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropDownButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropDownButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropDownButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropDownButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDynamicOverflowItemsChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlipViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusDisengagedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusEngagedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFocusEngagedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIcon2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIcon3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFontIconStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrame3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrame4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrame5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGrid2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGrid3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGrid4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewHeaderItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGridViewItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyle2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyleFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyleSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyleSelectorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupStyleSelectorOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingPanelClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingPanelOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingViewCandidatesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwritingViewTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHub(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSectionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSectionHeaderClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubSectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHubStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHyperlinkButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSourceElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSourceElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSourceElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIconSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCanvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCanvasFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbar2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbar3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarBallpointPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarBallpointPenButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPenButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPenButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPenFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomPenOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomToggleButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomToolButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomToolButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarCustomToolButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarEraserButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarEraserButton2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarEraserButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarEraserButtonStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarFlyoutItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarFlyoutItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarHighlighterButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarHighlighterButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarIsStencilButtonCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarMenuButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarMenuButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarMenuButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPencilButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarPencilButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarRulerButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarRulerButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarRulerButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStencilButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStencilButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarStencilButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarToggleButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarToolButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarToolButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkToolbarToolButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInsertionPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIsTextTrimmedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemContainerGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemContainerMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPanelTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsStackPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsStackPanel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsStackPanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsStackPanelStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsWrapGrid2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsWrapGridStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsWrapGridStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListPickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListPickerFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBase6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseHeaderItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewBaseStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewHeaderItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewPersistenceHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IListViewPersistenceHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaElementStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPlayerPresenterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControls2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControls3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControls4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaTransportControlsStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyout2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItem3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutPresenterStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSeparatorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSubItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSubItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSubItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuFlyoutSubItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewDisplayModeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemHeaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemInvokedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemSeparatorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewListFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewSelectionChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewTemplateSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INavigationViewTemplateSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPageStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPanel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParallaxView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParallaxViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParallaxViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBox5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxPasswordChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPasswordBoxStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIconSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIconSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPathIconStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersonPicture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersonPictureFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersonPictureStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerConfirmedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPickerFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivot(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivot2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivot3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPivotStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBarFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressRing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressRingStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadioButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemFontInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemFontInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemFontInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemImageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemImageInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemImageInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatingItemInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshContainerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshContainerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshInteractionRatioChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshVisualizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshVisualizerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRefreshVisualizerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativePanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativePanel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativePanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativePanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativePanelStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBox8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichEditBoxTextChangingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlock6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflow3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRichTextBlockStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRowDefinitionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollAnchorProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollContentPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollContentPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollContentPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewer4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScrollViewerViewChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISectionsInViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISectionsInViewChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoom(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISemanticZoomViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISettingsFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlider2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISliderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISliderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISliderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButtonClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitButtonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISplitViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanel4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanel5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanelStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanelStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStackPanelStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStyleSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStyleSelectorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStyleSelectorOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainBackgroundPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainBackgroundPanel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainPanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItemsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwipeItemsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIconSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIconSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISymbolIconStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlock7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBlockStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBox8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxBeforeTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextBoxTextChangingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCommandBarFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCommandBarFlyoutFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCompositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCompositionEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextControlCopyingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextControlCuttingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextControlPasteEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePicker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePicker3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerFlyoutStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleMenuFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSplitButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSplitButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSplitButtonFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSplitButtonIsCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitchOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToggleSwitchStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTip2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToolTipStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewCollapsedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewCollapsedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewExpandingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewExpandingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettingsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewListFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewNodeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewNodeStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITreeViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPaneView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPaneViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITwoPaneViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUIElementCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserControlFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariableSizedWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariableSizedWrapGridStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewbox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewboxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingPanelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingPanelOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingPanelProtected(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingStackPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingStackPanelOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualizingStackPanelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebView7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewBrushStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewFactory4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewNavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewSeparateProcessLostEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebViewWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWrapGridStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IconElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IconSourceElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Image(pub *mut ::core::ffi::c_void);
pub struct IncrementalLoadingTrigger(i32);
#[repr(transparent)]
pub struct InkCanvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarBallpointPenButton(pub *mut ::core::ffi::c_void);
pub struct InkToolbarButtonFlyoutPlacement(i32);
#[repr(transparent)]
pub struct InkToolbarCustomPen(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarCustomPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarCustomToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarCustomToolButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarEraserButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarFlyoutItem(pub *mut ::core::ffi::c_void);
pub struct InkToolbarFlyoutItemKind(i32);
#[repr(transparent)]
pub struct InkToolbarHighlighterButton(pub *mut ::core::ffi::c_void);
pub struct InkToolbarInitialControls(i32);
#[repr(transparent)]
pub struct InkToolbarIsStencilButtonCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarMenuButton(pub *mut ::core::ffi::c_void);
pub struct InkToolbarMenuKind(i32);
#[repr(transparent)]
pub struct InkToolbarPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarPenConfigurationControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarPencilButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarRulerButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarStencilButton(pub *mut ::core::ffi::c_void);
pub struct InkToolbarStencilKind(i32);
pub struct InkToolbarToggle(i32);
#[repr(transparent)]
pub struct InkToolbarToggleButton(pub *mut ::core::ffi::c_void);
pub struct InkToolbarTool(i32);
#[repr(transparent)]
pub struct InkToolbarToolButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IsTextTrimmedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemClickEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemContainerGenerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsPanelTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsPickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemsStackPanel(pub *mut ::core::ffi::c_void);
pub struct ItemsUpdatingScrollMode(i32);
#[repr(transparent)]
pub struct ItemsWrapGrid(pub *mut ::core::ffi::c_void);
pub struct LightDismissOverlayMode(i32);
#[repr(transparent)]
pub struct ListBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListBoxItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
pub struct ListPickerFlyoutSelectionMode(i32);
#[repr(transparent)]
pub struct ListView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewBaseHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewHeaderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewItemToKeyHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewKeyToItemHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListViewPersistenceHelper(pub *mut ::core::ffi::c_void);
pub struct ListViewReorderMode(i32);
pub struct ListViewSelectionMode(i32);
#[repr(transparent)]
pub struct MediaElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPlayerPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTransportControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaTransportControlsHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuBarItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuBarItemFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutItemBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuFlyoutSubItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationView(pub *mut ::core::ffi::c_void);
pub struct NavigationViewBackButtonVisible(i32);
#[repr(transparent)]
pub struct NavigationViewBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct NavigationViewDisplayMode(i32);
#[repr(transparent)]
pub struct NavigationViewDisplayModeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemHeader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewItemSeparator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewList(pub *mut ::core::ffi::c_void);
pub struct NavigationViewOverflowLabelMode(i32);
#[repr(transparent)]
pub struct NavigationViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
pub struct NavigationViewPaneDisplayMode(i32);
#[repr(transparent)]
pub struct NavigationViewSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct NavigationViewSelectionFollowsFocus(i32);
pub struct NavigationViewShoulderNavigationEnabled(i32);
#[repr(transparent)]
pub struct NavigationViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyEventHandler(pub *mut ::core::ffi::c_void);
pub struct Orientation(i32);
#[repr(transparent)]
pub struct Page(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Panel(pub *mut ::core::ffi::c_void);
pub struct PanelScrollingDirection(i32);
pub struct ParallaxSourceOffsetKind(i32);
#[repr(transparent)]
pub struct ParallaxView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordBoxPasswordChangingEventArgs(pub *mut ::core::ffi::c_void);
pub struct PasswordRevealMode(i32);
#[repr(transparent)]
pub struct PathIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PathIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PersonPicture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerConfirmedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Pivot(pub *mut ::core::ffi::c_void);
pub struct PivotHeaderFocusVisualPlacement(i32);
#[repr(transparent)]
pub struct PivotItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotItemEventArgs(pub *mut ::core::ffi::c_void);
pub struct PivotSlideInAnimationGroup(i32);
#[repr(transparent)]
pub struct ProgressBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProgressRing(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadioButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatingControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatingItemFontInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatingItemImageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatingItemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshInteractionRatioChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct RefreshPullDirection(i32);
#[repr(transparent)]
pub struct RefreshRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshVisualizer(pub *mut ::core::ffi::c_void);
pub struct RefreshVisualizerOrientation(i32);
pub struct RefreshVisualizerState(i32);
#[repr(transparent)]
pub struct RelativePanel(pub *mut ::core::ffi::c_void);
pub struct RequiresPointer(i32);
#[repr(transparent)]
pub struct RichEditBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
pub struct RichEditClipboardFormat(i32);
#[repr(transparent)]
pub struct RichTextBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichTextBlockOverflow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RowDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RowDefinitionCollection(pub *mut ::core::ffi::c_void);
pub struct ScrollBarVisibility(i32);
#[repr(transparent)]
pub struct ScrollContentPresenter(pub *mut ::core::ffi::c_void);
pub struct ScrollIntoViewAlignment(i32);
pub struct ScrollMode(i32);
#[repr(transparent)]
pub struct ScrollViewer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollViewerView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollViewerViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollViewerViewChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBoxQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBoxResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SearchBoxSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SectionsInViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SectionsInViewChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SelectionChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct SelectionMode(i32);
#[repr(transparent)]
pub struct SemanticZoom(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SemanticZoomLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SemanticZoomViewChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SemanticZoomViewChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SettingsFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Slider(pub *mut ::core::ffi::c_void);
pub struct SnapPointsType(i32);
#[repr(transparent)]
pub struct SplitButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitButtonClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitView(pub *mut ::core::ffi::c_void);
pub struct SplitViewDisplayMode(i32);
#[repr(transparent)]
pub struct SplitViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
pub struct SplitViewPanePlacement(i32);
#[repr(transparent)]
pub struct StackPanel(pub *mut ::core::ffi::c_void);
pub struct StretchDirection(i32);
#[repr(transparent)]
pub struct StyleSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwapChainBackgroundPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwapChainPanel(pub *mut ::core::ffi::c_void);
pub struct SwipeBehaviorOnInvoked(i32);
#[repr(transparent)]
pub struct SwipeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItems(pub *mut ::core::ffi::c_void);
pub struct SwipeMode(i32);
pub struct Symbol(i32);
#[repr(transparent)]
pub struct SymbolIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SymbolIconSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBoxBeforeTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextCommandBarFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextCompositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextCompositionEndedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextControlCopyingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextControlCuttingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextControlPasteEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TextControlPasteEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleMenuFlyoutItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSplitButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSplitButtonIsCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToggleSwitch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToolTip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToolTipService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewCollapsedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewExpandingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TreeViewNode(pub *mut ::core::ffi::c_void);
pub struct TreeViewSelectionMode(i32);
#[repr(transparent)]
pub struct TwoPaneView(pub *mut ::core::ffi::c_void);
pub struct TwoPaneViewMode(i32);
pub struct TwoPaneViewPriority(i32);
pub struct TwoPaneViewTallModeConfiguration(i32);
pub struct TwoPaneViewWideModeConfiguration(i32);
#[repr(transparent)]
pub struct UIElementCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VariableSizedWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Viewbox(pub *mut ::core::ffi::c_void);
pub struct VirtualizationMode(i32);
#[repr(transparent)]
pub struct VirtualizingPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VirtualizingStackPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewBrush(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
pub struct WebViewExecutionMode(i32);
#[repr(transparent)]
pub struct WebViewLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewNavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewNavigationFailedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewPermissionRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct WebViewPermissionState(i32);
pub struct WebViewPermissionType(i32);
#[repr(transparent)]
pub struct WebViewSeparateProcessLostEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebViewWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WrapGrid(pub *mut ::core::ffi::c_void);
pub struct ZoomMode(i32);
