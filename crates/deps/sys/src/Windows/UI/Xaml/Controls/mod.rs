#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct AppBarClosedDisplayMode(pub i32);
impl AppBarClosedDisplayMode {
    pub const Compact: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for AppBarClosedDisplayMode {}
impl ::core::clone::Clone for AppBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct AutoSuggestionBoxTextChangeReason(pub i32);
impl AutoSuggestionBoxTextChangeReason {
    pub const UserInput: Self = Self(0i32);
    pub const ProgrammaticChange: Self = Self(1i32);
    pub const SuggestionChosen: Self = Self(2i32);
}
impl ::core::marker::Copy for AutoSuggestionBoxTextChangeReason {}
impl ::core::clone::Clone for AutoSuggestionBoxTextChangeReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackClickEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundSizing(pub i32);
impl BackgroundSizing {
    pub const InnerBorderEdge: Self = Self(0i32);
    pub const OuterBorderEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundSizing {}
impl ::core::clone::Clone for BackgroundSizing {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct CalendarViewDisplayMode(pub i32);
impl CalendarViewDisplayMode {
    pub const Month: Self = Self(0i32);
    pub const Year: Self = Self(1i32);
    pub const Decade: Self = Self(2i32);
}
impl ::core::marker::Copy for CalendarViewDisplayMode {}
impl ::core::clone::Clone for CalendarViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarViewSelectedDatesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CalendarViewSelectionMode(pub i32);
impl CalendarViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for CalendarViewSelectionMode {}
impl ::core::clone::Clone for CalendarViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CandidateWindowAlignment(pub i32);
impl CandidateWindowAlignment {
    pub const Default: Self = Self(0i32);
    pub const BottomEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for CandidateWindowAlignment {}
impl ::core::clone::Clone for CandidateWindowAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CandidateWindowBoundsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Canvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CaptureElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CharacterCasing(pub i32);
impl CharacterCasing {
    pub const Normal: Self = Self(0i32);
    pub const Lower: Self = Self(1i32);
    pub const Upper: Self = Self(2i32);
}
impl ::core::marker::Copy for CharacterCasing {}
impl ::core::clone::Clone for CharacterCasing {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ClickMode(pub i32);
impl ClickMode {
    pub const Release: Self = Self(0i32);
    pub const Press: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
}
impl ::core::marker::Copy for ClickMode {}
impl ::core::clone::Clone for ClickMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorPickerHsvChannel(pub i32);
impl ColorPickerHsvChannel {
    pub const Hue: Self = Self(0i32);
    pub const Saturation: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
    pub const Alpha: Self = Self(3i32);
}
impl ::core::marker::Copy for ColorPickerHsvChannel {}
impl ::core::clone::Clone for ColorPickerHsvChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorSpectrumComponents(pub i32);
impl ColorSpectrumComponents {
    pub const HueValue: Self = Self(0i32);
    pub const ValueHue: Self = Self(1i32);
    pub const HueSaturation: Self = Self(2i32);
    pub const SaturationHue: Self = Self(3i32);
    pub const SaturationValue: Self = Self(4i32);
    pub const ValueSaturation: Self = Self(5i32);
}
impl ::core::marker::Copy for ColorSpectrumComponents {}
impl ::core::clone::Clone for ColorSpectrumComponents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorSpectrumShape(pub i32);
impl ColorSpectrumShape {
    pub const Box: Self = Self(0i32);
    pub const Ring: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorSpectrumShape {}
impl ::core::clone::Clone for ColorSpectrumShape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColumnDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColumnDefinitionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ComboBoxSelectionChangedTrigger(pub i32);
impl ComboBoxSelectionChangedTrigger {
    pub const Committed: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for ComboBoxSelectionChangedTrigger {}
impl ::core::clone::Clone for ComboBoxSelectionChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBarDefaultLabelPosition(pub i32);
impl CommandBarDefaultLabelPosition {
    pub const Bottom: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl ::core::marker::Copy for CommandBarDefaultLabelPosition {}
impl ::core::clone::Clone for CommandBarDefaultLabelPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarDynamicOverflowAction(pub i32);
impl CommandBarDynamicOverflowAction {
    pub const AddingToOverflow: Self = Self(0i32);
    pub const RemovingFromOverflow: Self = Self(1i32);
}
impl ::core::marker::Copy for CommandBarDynamicOverflowAction {}
impl ::core::clone::Clone for CommandBarDynamicOverflowAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommandBarLabelPosition(pub i32);
impl CommandBarLabelPosition {
    pub const Default: Self = Self(0i32);
    pub const Collapsed: Self = Self(1i32);
}
impl ::core::marker::Copy for CommandBarLabelPosition {}
impl ::core::clone::Clone for CommandBarLabelPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarOverflowButtonVisibility(pub i32);
impl CommandBarOverflowButtonVisibility {
    pub const Auto: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Collapsed: Self = Self(2i32);
}
impl ::core::marker::Copy for CommandBarOverflowButtonVisibility {}
impl ::core::clone::Clone for CommandBarOverflowButtonVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBarOverflowPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContainerContentChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentDialogButton(pub i32);
impl ContentDialogButton {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Secondary: Self = Self(2i32);
    pub const Close: Self = Self(3i32);
}
impl ::core::marker::Copy for ContentDialogButton {}
impl ::core::clone::Clone for ContentDialogButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ContentDialogPlacement(pub i32);
impl ContentDialogPlacement {
    pub const Popup: Self = Self(0i32);
    pub const InPlace: Self = Self(1i32);
}
impl ::core::marker::Copy for ContentDialogPlacement {}
impl ::core::clone::Clone for ContentDialogPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogResult(pub i32);
impl ContentDialogResult {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Secondary: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentDialogResult {}
impl ::core::clone::Clone for ContentDialogResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentLinkChangeKind(pub i32);
impl ContentLinkChangeKind {
    pub const Inserted: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Edited: Self = Self(2i32);
}
impl ::core::marker::Copy for ContentLinkChangeKind {}
impl ::core::clone::Clone for ContentLinkChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct DisabledFormattingAccelerators(pub u32);
impl DisabledFormattingAccelerators {
    pub const None: Self = Self(0u32);
    pub const Bold: Self = Self(1u32);
    pub const Italic: Self = Self(2u32);
    pub const Underline: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for DisabledFormattingAccelerators {}
impl ::core::clone::Clone for DisabledFormattingAccelerators {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct HandwritingPanelPlacementAlignment(pub i32);
impl HandwritingPanelPlacementAlignment {
    pub const Auto: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const TopRight: Self = Self(2i32);
    pub const BottomLeft: Self = Self(3i32);
    pub const BottomRight: Self = Self(4i32);
}
impl ::core::marker::Copy for HandwritingPanelPlacementAlignment {}
impl ::core::clone::Clone for HandwritingPanelPlacementAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct IncrementalLoadingTrigger(pub i32);
impl IncrementalLoadingTrigger {
    pub const None: Self = Self(0i32);
    pub const Edge: Self = Self(1i32);
}
impl ::core::marker::Copy for IncrementalLoadingTrigger {}
impl ::core::clone::Clone for IncrementalLoadingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkCanvas(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarBallpointPenButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarButtonFlyoutPlacement(pub i32);
impl InkToolbarButtonFlyoutPlacement {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
}
impl ::core::marker::Copy for InkToolbarButtonFlyoutPlacement {}
impl ::core::clone::Clone for InkToolbarButtonFlyoutPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct InkToolbarFlyoutItemKind(pub i32);
impl InkToolbarFlyoutItemKind {
    pub const Simple: Self = Self(0i32);
    pub const Radio: Self = Self(1i32);
    pub const Check: Self = Self(2i32);
    pub const RadioCheck: Self = Self(3i32);
}
impl ::core::marker::Copy for InkToolbarFlyoutItemKind {}
impl ::core::clone::Clone for InkToolbarFlyoutItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarHighlighterButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarInitialControls(pub i32);
impl InkToolbarInitialControls {
    pub const All: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const PensOnly: Self = Self(2i32);
    pub const AllExceptPens: Self = Self(3i32);
}
impl ::core::marker::Copy for InkToolbarInitialControls {}
impl ::core::clone::Clone for InkToolbarInitialControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarIsStencilButtonCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarMenuButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarMenuKind(pub i32);
impl InkToolbarMenuKind {
    pub const Stencil: Self = Self(0i32);
}
impl ::core::marker::Copy for InkToolbarMenuKind {}
impl ::core::clone::Clone for InkToolbarMenuKind {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct InkToolbarStencilKind(pub i32);
impl InkToolbarStencilKind {
    pub const Ruler: Self = Self(0i32);
    pub const Protractor: Self = Self(1i32);
}
impl ::core::marker::Copy for InkToolbarStencilKind {}
impl ::core::clone::Clone for InkToolbarStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarToggle(pub i32);
impl InkToolbarToggle {
    pub const Ruler: Self = Self(0i32);
    pub const Custom: Self = Self(1i32);
}
impl ::core::marker::Copy for InkToolbarToggle {}
impl ::core::clone::Clone for InkToolbarToggle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarToggleButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkToolbarTool(pub i32);
impl InkToolbarTool {
    pub const BallpointPen: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
    pub const Highlighter: Self = Self(2i32);
    pub const Eraser: Self = Self(3i32);
    pub const CustomPen: Self = Self(4i32);
    pub const CustomTool: Self = Self(5i32);
}
impl ::core::marker::Copy for InkToolbarTool {}
impl ::core::clone::Clone for InkToolbarTool {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ItemsUpdatingScrollMode(pub i32);
impl ItemsUpdatingScrollMode {
    pub const KeepItemsInView: Self = Self(0i32);
    pub const KeepScrollOffset: Self = Self(1i32);
    pub const KeepLastItemInView: Self = Self(2i32);
}
impl ::core::marker::Copy for ItemsUpdatingScrollMode {}
impl ::core::clone::Clone for ItemsUpdatingScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LightDismissOverlayMode(pub i32);
impl LightDismissOverlayMode {
    pub const Auto: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
}
impl ::core::marker::Copy for LightDismissOverlayMode {}
impl ::core::clone::Clone for LightDismissOverlayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListBoxItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyout(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ListPickerFlyoutSelectionMode(pub i32);
impl ListPickerFlyoutSelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
}
impl ::core::marker::Copy for ListPickerFlyoutSelectionMode {}
impl ::core::clone::Clone for ListPickerFlyoutSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ListViewReorderMode(pub i32);
impl ListViewReorderMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewReorderMode {}
impl ::core::clone::Clone for ListViewReorderMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewSelectionMode(pub i32);
impl ListViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
    pub const Extended: Self = Self(3i32);
}
impl ::core::marker::Copy for ListViewSelectionMode {}
impl ::core::clone::Clone for ListViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct NavigationViewBackButtonVisible(pub i32);
impl NavigationViewBackButtonVisible {
    pub const Collapsed: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewBackButtonVisible {}
impl ::core::clone::Clone for NavigationViewBackButtonVisible {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewDisplayMode(pub i32);
impl NavigationViewDisplayMode {
    pub const Minimal: Self = Self(0i32);
    pub const Compact: Self = Self(1i32);
    pub const Expanded: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewDisplayMode {}
impl ::core::clone::Clone for NavigationViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct NavigationViewOverflowLabelMode(pub i32);
impl NavigationViewOverflowLabelMode {
    pub const MoreLabel: Self = Self(0i32);
    pub const NoLabel: Self = Self(1i32);
}
impl ::core::marker::Copy for NavigationViewOverflowLabelMode {}
impl ::core::clone::Clone for NavigationViewOverflowLabelMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewPaneDisplayMode(pub i32);
impl NavigationViewPaneDisplayMode {
    pub const Auto: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Top: Self = Self(2i32);
    pub const LeftCompact: Self = Self(3i32);
    pub const LeftMinimal: Self = Self(4i32);
}
impl ::core::marker::Copy for NavigationViewPaneDisplayMode {}
impl ::core::clone::Clone for NavigationViewPaneDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NavigationViewSelectionFollowsFocus(pub i32);
impl NavigationViewSelectionFollowsFocus {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NavigationViewSelectionFollowsFocus {}
impl ::core::clone::Clone for NavigationViewSelectionFollowsFocus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewShoulderNavigationEnabled(pub i32);
impl NavigationViewShoulderNavigationEnabled {
    pub const WhenSelectionFollowsFocus: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for NavigationViewShoulderNavigationEnabled {}
impl ::core::clone::Clone for NavigationViewShoulderNavigationEnabled {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewTemplateSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Orientation(pub i32);
impl Orientation {
    pub const Vertical: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
}
impl ::core::marker::Copy for Orientation {}
impl ::core::clone::Clone for Orientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Page(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Panel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PanelScrollingDirection(pub i32);
impl PanelScrollingDirection {
    pub const None: Self = Self(0i32);
    pub const Forward: Self = Self(1i32);
    pub const Backward: Self = Self(2i32);
}
impl ::core::marker::Copy for PanelScrollingDirection {}
impl ::core::clone::Clone for PanelScrollingDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ParallaxSourceOffsetKind(pub i32);
impl ParallaxSourceOffsetKind {
    pub const Absolute: Self = Self(0i32);
    pub const Relative: Self = Self(1i32);
}
impl ::core::marker::Copy for ParallaxSourceOffsetKind {}
impl ::core::clone::Clone for ParallaxSourceOffsetKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ParallaxView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordBoxPasswordChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PasswordRevealMode(pub i32);
impl PasswordRevealMode {
    pub const Peek: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
}
impl ::core::marker::Copy for PasswordRevealMode {}
impl ::core::clone::Clone for PasswordRevealMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PivotHeaderFocusVisualPlacement(pub i32);
impl PivotHeaderFocusVisualPlacement {
    pub const ItemHeaders: Self = Self(0i32);
    pub const SelectedItemHeader: Self = Self(1i32);
}
impl ::core::marker::Copy for PivotHeaderFocusVisualPlacement {}
impl ::core::clone::Clone for PivotHeaderFocusVisualPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotItemEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PivotSlideInAnimationGroup(pub i32);
impl PivotSlideInAnimationGroup {
    pub const Default: Self = Self(0i32);
    pub const GroupOne: Self = Self(1i32);
    pub const GroupTwo: Self = Self(2i32);
    pub const GroupThree: Self = Self(3i32);
}
impl ::core::marker::Copy for PivotSlideInAnimationGroup {}
impl ::core::clone::Clone for PivotSlideInAnimationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct RefreshPullDirection(pub i32);
impl RefreshPullDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const TopToBottom: Self = Self(1i32);
    pub const RightToLeft: Self = Self(2i32);
    pub const BottomToTop: Self = Self(3i32);
}
impl ::core::marker::Copy for RefreshPullDirection {}
impl ::core::clone::Clone for RefreshPullDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshVisualizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RefreshVisualizerOrientation(pub i32);
impl RefreshVisualizerOrientation {
    pub const Auto: Self = Self(0i32);
    pub const Normal: Self = Self(1i32);
    pub const Rotate90DegreesCounterclockwise: Self = Self(2i32);
    pub const Rotate270DegreesCounterclockwise: Self = Self(3i32);
}
impl ::core::marker::Copy for RefreshVisualizerOrientation {}
impl ::core::clone::Clone for RefreshVisualizerOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshVisualizerState(pub i32);
impl RefreshVisualizerState {
    pub const Idle: Self = Self(0i32);
    pub const Peeking: Self = Self(1i32);
    pub const Interacting: Self = Self(2i32);
    pub const Pending: Self = Self(3i32);
    pub const Refreshing: Self = Self(4i32);
}
impl ::core::marker::Copy for RefreshVisualizerState {}
impl ::core::clone::Clone for RefreshVisualizerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RelativePanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RequiresPointer(pub i32);
impl RequiresPointer {
    pub const Never: Self = Self(0i32);
    pub const WhenEngaged: Self = Self(1i32);
    pub const WhenFocused: Self = Self(2i32);
}
impl ::core::marker::Copy for RequiresPointer {}
impl ::core::clone::Clone for RequiresPointer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichEditBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichEditClipboardFormat(pub i32);
impl RichEditClipboardFormat {
    pub const AllFormats: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for RichEditClipboardFormat {}
impl ::core::clone::Clone for RichEditClipboardFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichTextBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RichTextBlockOverflow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RowDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RowDefinitionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollBarVisibility(pub i32);
impl ScrollBarVisibility {
    pub const Disabled: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
    pub const Visible: Self = Self(3i32);
}
impl ::core::marker::Copy for ScrollBarVisibility {}
impl ::core::clone::Clone for ScrollBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollContentPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ScrollIntoViewAlignment(pub i32);
impl ScrollIntoViewAlignment {
    pub const Default: Self = Self(0i32);
    pub const Leading: Self = Self(1i32);
}
impl ::core::marker::Copy for ScrollIntoViewAlignment {}
impl ::core::clone::Clone for ScrollIntoViewAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollMode(pub i32);
impl ScrollMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const Auto: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollMode {}
impl ::core::clone::Clone for ScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SelectionMode(pub i32);
impl SelectionMode {
    pub const Single: Self = Self(0i32);
    pub const Multiple: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for SelectionMode {}
impl ::core::clone::Clone for SelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SnapPointsType(pub i32);
impl SnapPointsType {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Mandatory: Self = Self(2i32);
    pub const OptionalSingle: Self = Self(3i32);
    pub const MandatorySingle: Self = Self(4i32);
}
impl ::core::marker::Copy for SnapPointsType {}
impl ::core::clone::Clone for SnapPointsType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitButtonClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitViewDisplayMode(pub i32);
impl SplitViewDisplayMode {
    pub const Overlay: Self = Self(0i32);
    pub const Inline: Self = Self(1i32);
    pub const CompactOverlay: Self = Self(2i32);
    pub const CompactInline: Self = Self(3i32);
}
impl ::core::marker::Copy for SplitViewDisplayMode {}
impl ::core::clone::Clone for SplitViewDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SplitViewPanePlacement(pub i32);
impl SplitViewPanePlacement {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
}
impl ::core::marker::Copy for SplitViewPanePlacement {}
impl ::core::clone::Clone for SplitViewPanePlacement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StackPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StretchDirection(pub i32);
impl StretchDirection {
    pub const UpOnly: Self = Self(0i32);
    pub const DownOnly: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl ::core::marker::Copy for StretchDirection {}
impl ::core::clone::Clone for StretchDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StyleSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwapChainBackgroundPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwapChainPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeBehaviorOnInvoked(pub i32);
impl SwipeBehaviorOnInvoked {
    pub const Auto: Self = Self(0i32);
    pub const Close: Self = Self(1i32);
    pub const RemainOpen: Self = Self(2i32);
}
impl ::core::marker::Copy for SwipeBehaviorOnInvoked {}
impl ::core::clone::Clone for SwipeBehaviorOnInvoked {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SwipeMode(pub i32);
impl SwipeMode {
    pub const Reveal: Self = Self(0i32);
    pub const Execute: Self = Self(1i32);
}
impl ::core::marker::Copy for SwipeMode {}
impl ::core::clone::Clone for SwipeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Symbol(pub i32);
impl Symbol {
    pub const Previous: Self = Self(57600i32);
    pub const Next: Self = Self(57601i32);
    pub const Play: Self = Self(57602i32);
    pub const Pause: Self = Self(57603i32);
    pub const Edit: Self = Self(57604i32);
    pub const Save: Self = Self(57605i32);
    pub const Clear: Self = Self(57606i32);
    pub const Delete: Self = Self(57607i32);
    pub const Remove: Self = Self(57608i32);
    pub const Add: Self = Self(57609i32);
    pub const Cancel: Self = Self(57610i32);
    pub const Accept: Self = Self(57611i32);
    pub const More: Self = Self(57612i32);
    pub const Redo: Self = Self(57613i32);
    pub const Undo: Self = Self(57614i32);
    pub const Home: Self = Self(57615i32);
    pub const Up: Self = Self(57616i32);
    pub const Forward: Self = Self(57617i32);
    pub const Back: Self = Self(57618i32);
    pub const Favorite: Self = Self(57619i32);
    pub const Camera: Self = Self(57620i32);
    pub const Setting: Self = Self(57621i32);
    pub const Video: Self = Self(57622i32);
    pub const Sync: Self = Self(57623i32);
    pub const Download: Self = Self(57624i32);
    pub const Mail: Self = Self(57625i32);
    pub const Find: Self = Self(57626i32);
    pub const Help: Self = Self(57627i32);
    pub const Upload: Self = Self(57628i32);
    pub const Emoji: Self = Self(57629i32);
    pub const TwoPage: Self = Self(57630i32);
    pub const LeaveChat: Self = Self(57631i32);
    pub const MailForward: Self = Self(57632i32);
    pub const Clock: Self = Self(57633i32);
    pub const Send: Self = Self(57634i32);
    pub const Crop: Self = Self(57635i32);
    pub const RotateCamera: Self = Self(57636i32);
    pub const People: Self = Self(57637i32);
    pub const OpenPane: Self = Self(57638i32);
    pub const ClosePane: Self = Self(57639i32);
    pub const World: Self = Self(57640i32);
    pub const Flag: Self = Self(57641i32);
    pub const PreviewLink: Self = Self(57642i32);
    pub const Globe: Self = Self(57643i32);
    pub const Trim: Self = Self(57644i32);
    pub const AttachCamera: Self = Self(57645i32);
    pub const ZoomIn: Self = Self(57646i32);
    pub const Bookmarks: Self = Self(57647i32);
    pub const Document: Self = Self(57648i32);
    pub const ProtectedDocument: Self = Self(57649i32);
    pub const Page: Self = Self(57650i32);
    pub const Bullets: Self = Self(57651i32);
    pub const Comment: Self = Self(57652i32);
    pub const MailFilled: Self = Self(57653i32);
    pub const ContactInfo: Self = Self(57654i32);
    pub const HangUp: Self = Self(57655i32);
    pub const ViewAll: Self = Self(57656i32);
    pub const MapPin: Self = Self(57657i32);
    pub const Phone: Self = Self(57658i32);
    pub const VideoChat: Self = Self(57659i32);
    pub const Switch: Self = Self(57660i32);
    pub const Contact: Self = Self(57661i32);
    pub const Rename: Self = Self(57662i32);
    pub const Pin: Self = Self(57665i32);
    pub const MusicInfo: Self = Self(57666i32);
    pub const Go: Self = Self(57667i32);
    pub const Keyboard: Self = Self(57668i32);
    pub const DockLeft: Self = Self(57669i32);
    pub const DockRight: Self = Self(57670i32);
    pub const DockBottom: Self = Self(57671i32);
    pub const Remote: Self = Self(57672i32);
    pub const Refresh: Self = Self(57673i32);
    pub const Rotate: Self = Self(57674i32);
    pub const Shuffle: Self = Self(57675i32);
    pub const List: Self = Self(57676i32);
    pub const Shop: Self = Self(57677i32);
    pub const SelectAll: Self = Self(57678i32);
    pub const Orientation: Self = Self(57679i32);
    pub const Import: Self = Self(57680i32);
    pub const ImportAll: Self = Self(57681i32);
    pub const BrowsePhotos: Self = Self(57685i32);
    pub const WebCam: Self = Self(57686i32);
    pub const Pictures: Self = Self(57688i32);
    pub const SaveLocal: Self = Self(57689i32);
    pub const Caption: Self = Self(57690i32);
    pub const Stop: Self = Self(57691i32);
    pub const ShowResults: Self = Self(57692i32);
    pub const Volume: Self = Self(57693i32);
    pub const Repair: Self = Self(57694i32);
    pub const Message: Self = Self(57695i32);
    pub const Page2: Self = Self(57696i32);
    pub const CalendarDay: Self = Self(57697i32);
    pub const CalendarWeek: Self = Self(57698i32);
    pub const Calendar: Self = Self(57699i32);
    pub const Character: Self = Self(57700i32);
    pub const MailReplyAll: Self = Self(57701i32);
    pub const Read: Self = Self(57702i32);
    pub const Link: Self = Self(57703i32);
    pub const Account: Self = Self(57704i32);
    pub const ShowBcc: Self = Self(57705i32);
    pub const HideBcc: Self = Self(57706i32);
    pub const Cut: Self = Self(57707i32);
    pub const Attach: Self = Self(57708i32);
    pub const Paste: Self = Self(57709i32);
    pub const Filter: Self = Self(57710i32);
    pub const Copy: Self = Self(57711i32);
    pub const Emoji2: Self = Self(57712i32);
    pub const Important: Self = Self(57713i32);
    pub const MailReply: Self = Self(57714i32);
    pub const SlideShow: Self = Self(57715i32);
    pub const Sort: Self = Self(57716i32);
    pub const Manage: Self = Self(57720i32);
    pub const AllApps: Self = Self(57721i32);
    pub const DisconnectDrive: Self = Self(57722i32);
    pub const MapDrive: Self = Self(57723i32);
    pub const NewWindow: Self = Self(57724i32);
    pub const OpenWith: Self = Self(57725i32);
    pub const ContactPresence: Self = Self(57729i32);
    pub const Priority: Self = Self(57730i32);
    pub const GoToToday: Self = Self(57732i32);
    pub const Font: Self = Self(57733i32);
    pub const FontColor: Self = Self(57734i32);
    pub const Contact2: Self = Self(57735i32);
    pub const Folder: Self = Self(57736i32);
    pub const Audio: Self = Self(57737i32);
    pub const Placeholder: Self = Self(57738i32);
    pub const View: Self = Self(57739i32);
    pub const SetLockScreen: Self = Self(57740i32);
    pub const SetTile: Self = Self(57741i32);
    pub const ClosedCaption: Self = Self(57744i32);
    pub const StopSlideShow: Self = Self(57745i32);
    pub const Permissions: Self = Self(57746i32);
    pub const Highlight: Self = Self(57747i32);
    pub const DisableUpdates: Self = Self(57748i32);
    pub const UnFavorite: Self = Self(57749i32);
    pub const UnPin: Self = Self(57750i32);
    pub const OpenLocal: Self = Self(57751i32);
    pub const Mute: Self = Self(57752i32);
    pub const Italic: Self = Self(57753i32);
    pub const Underline: Self = Self(57754i32);
    pub const Bold: Self = Self(57755i32);
    pub const MoveToFolder: Self = Self(57756i32);
    pub const LikeDislike: Self = Self(57757i32);
    pub const Dislike: Self = Self(57758i32);
    pub const Like: Self = Self(57759i32);
    pub const AlignRight: Self = Self(57760i32);
    pub const AlignCenter: Self = Self(57761i32);
    pub const AlignLeft: Self = Self(57762i32);
    pub const Zoom: Self = Self(57763i32);
    pub const ZoomOut: Self = Self(57764i32);
    pub const OpenFile: Self = Self(57765i32);
    pub const OtherUser: Self = Self(57766i32);
    pub const Admin: Self = Self(57767i32);
    pub const Street: Self = Self(57795i32);
    pub const Map: Self = Self(57796i32);
    pub const ClearSelection: Self = Self(57797i32);
    pub const FontDecrease: Self = Self(57798i32);
    pub const FontIncrease: Self = Self(57799i32);
    pub const FontSize: Self = Self(57800i32);
    pub const CellPhone: Self = Self(57801i32);
    pub const ReShare: Self = Self(57802i32);
    pub const Tag: Self = Self(57803i32);
    pub const RepeatOne: Self = Self(57804i32);
    pub const RepeatAll: Self = Self(57805i32);
    pub const OutlineStar: Self = Self(57806i32);
    pub const SolidStar: Self = Self(57807i32);
    pub const Calculator: Self = Self(57808i32);
    pub const Directions: Self = Self(57809i32);
    pub const Target: Self = Self(57810i32);
    pub const Library: Self = Self(57811i32);
    pub const PhoneBook: Self = Self(57812i32);
    pub const Memo: Self = Self(57813i32);
    pub const Microphone: Self = Self(57814i32);
    pub const PostUpdate: Self = Self(57815i32);
    pub const BackToWindow: Self = Self(57816i32);
    pub const FullScreen: Self = Self(57817i32);
    pub const NewFolder: Self = Self(57818i32);
    pub const CalendarReply: Self = Self(57819i32);
    pub const UnSyncFolder: Self = Self(57821i32);
    pub const ReportHacked: Self = Self(57822i32);
    pub const SyncFolder: Self = Self(57823i32);
    pub const BlockContact: Self = Self(57824i32);
    pub const SwitchApps: Self = Self(57825i32);
    pub const AddFriend: Self = Self(57826i32);
    pub const TouchPointer: Self = Self(57827i32);
    pub const GoToStart: Self = Self(57828i32);
    pub const ZeroBars: Self = Self(57829i32);
    pub const OneBar: Self = Self(57830i32);
    pub const TwoBars: Self = Self(57831i32);
    pub const ThreeBars: Self = Self(57832i32);
    pub const FourBars: Self = Self(57833i32);
    pub const Scan: Self = Self(58004i32);
    pub const Preview: Self = Self(58005i32);
    pub const GlobalNavigationButton: Self = Self(59136i32);
    pub const Share: Self = Self(59181i32);
    pub const Print: Self = Self(59209i32);
    pub const XboxOneConsole: Self = Self(59792i32);
}
impl ::core::marker::Copy for Symbol {}
impl ::core::clone::Clone for Symbol {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct TreeViewSelectionMode(pub i32);
impl TreeViewSelectionMode {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for TreeViewSelectionMode {}
impl ::core::clone::Clone for TreeViewSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPaneView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TwoPaneViewMode(pub i32);
impl TwoPaneViewMode {
    pub const SinglePane: Self = Self(0i32);
    pub const Wide: Self = Self(1i32);
    pub const Tall: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewMode {}
impl ::core::clone::Clone for TwoPaneViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPaneViewPriority(pub i32);
impl TwoPaneViewPriority {
    pub const Pane1: Self = Self(0i32);
    pub const Pane2: Self = Self(1i32);
}
impl ::core::marker::Copy for TwoPaneViewPriority {}
impl ::core::clone::Clone for TwoPaneViewPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPaneViewTallModeConfiguration(pub i32);
impl TwoPaneViewTallModeConfiguration {
    pub const SinglePane: Self = Self(0i32);
    pub const TopBottom: Self = Self(1i32);
    pub const BottomTop: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewTallModeConfiguration {}
impl ::core::clone::Clone for TwoPaneViewTallModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TwoPaneViewWideModeConfiguration(pub i32);
impl TwoPaneViewWideModeConfiguration {
    pub const SinglePane: Self = Self(0i32);
    pub const LeftRight: Self = Self(1i32);
    pub const RightLeft: Self = Self(2i32);
}
impl ::core::marker::Copy for TwoPaneViewWideModeConfiguration {}
impl ::core::clone::Clone for TwoPaneViewWideModeConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UIElementCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VariableSizedWrapGrid(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Viewbox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VirtualizationMode(pub i32);
impl VirtualizationMode {
    pub const Standard: Self = Self(0i32);
    pub const Recycling: Self = Self(1i32);
}
impl ::core::marker::Copy for VirtualizationMode {}
impl ::core::clone::Clone for VirtualizationMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct WebViewExecutionMode(pub i32);
impl WebViewExecutionMode {
    pub const SameThread: Self = Self(0i32);
    pub const SeparateThread: Self = Self(1i32);
    pub const SeparateProcess: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewExecutionMode {}
impl ::core::clone::Clone for WebViewExecutionMode {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct WebViewPermissionState(pub i32);
impl WebViewPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewPermissionState {}
impl ::core::clone::Clone for WebViewPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewPermissionType(pub i32);
impl WebViewPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewPermissionType {}
impl ::core::clone::Clone for WebViewPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct ZoomMode(pub i32);
impl ZoomMode {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
impl ::core::marker::Copy for ZoomMode {}
impl ::core::clone::Clone for ZoomMode {
    fn clone(&self) -> Self {
        *self
    }
}
