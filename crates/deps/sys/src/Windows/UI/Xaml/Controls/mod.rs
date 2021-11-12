#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Xaml_Controls_Maps")]
pub mod Maps;
#[cfg(feature = "UI_Xaml_Controls_Primitives")]
pub mod Primitives;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnchorRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AnchorRequestedEventArgs {}
impl ::core::clone::Clone for AnchorRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBar {}
impl ::core::clone::Clone for AppBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarButton {}
impl ::core::clone::Clone for AppBarButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AppBarElementContainer {}
impl ::core::clone::Clone for AppBarElementContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarSeparator {}
impl ::core::clone::Clone for AppBarSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBarToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBarToggleButton {}
impl ::core::clone::Clone for AppBarToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSuggestBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoSuggestBox {}
impl ::core::clone::Clone for AutoSuggestBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSuggestBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoSuggestBoxQuerySubmittedEventArgs {}
impl ::core::clone::Clone for AutoSuggestBoxQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSuggestBoxSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoSuggestBoxSuggestionChosenEventArgs {}
impl ::core::clone::Clone for AutoSuggestBoxSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutoSuggestBoxTextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutoSuggestBoxTextChangedEventArgs {}
impl ::core::clone::Clone for AutoSuggestBoxTextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for BackClickEventArgs {}
impl ::core::clone::Clone for BackClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackClickEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackClickEventHandler {}
impl ::core::clone::Clone for BackClickEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for BitmapIcon {}
impl ::core::clone::Clone for BitmapIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BitmapIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BitmapIconSource {}
impl ::core::clone::Clone for BitmapIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Border(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Border {}
impl ::core::clone::Clone for Border {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Button(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Button {}
impl ::core::clone::Clone for Button {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarDatePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarDatePicker {}
impl ::core::clone::Clone for CalendarDatePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarDatePickerDateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarDatePickerDateChangedEventArgs {}
impl ::core::clone::Clone for CalendarDatePickerDateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarView {}
impl ::core::clone::Clone for CalendarView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarViewDayItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarViewDayItem {}
impl ::core::clone::Clone for CalendarViewDayItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarViewDayItemChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarViewDayItemChangingEventArgs {}
impl ::core::clone::Clone for CalendarViewDayItemChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CalendarViewDayItemChangingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CalendarViewDayItemChangingEventHandler {}
impl ::core::clone::Clone for CalendarViewDayItemChangingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CalendarViewSelectedDatesChangedEventArgs {}
impl ::core::clone::Clone for CalendarViewSelectedDatesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CandidateWindowBoundsChangedEventArgs {}
impl ::core::clone::Clone for CandidateWindowBoundsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Canvas(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Canvas {}
impl ::core::clone::Clone for Canvas {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CaptureElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CaptureElement {}
impl ::core::clone::Clone for CaptureElement {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CheckBox {}
impl ::core::clone::Clone for CheckBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChoosingGroupHeaderContainerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChoosingGroupHeaderContainerEventArgs {}
impl ::core::clone::Clone for ChoosingGroupHeaderContainerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChoosingItemContainerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChoosingItemContainerEventArgs {}
impl ::core::clone::Clone for ChoosingItemContainerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CleanUpVirtualizedItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CleanUpVirtualizedItemEventArgs {}
impl ::core::clone::Clone for CleanUpVirtualizedItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CleanUpVirtualizedItemEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CleanUpVirtualizedItemEventHandler {}
impl ::core::clone::Clone for CleanUpVirtualizedItemEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ColorChangedEventArgs {}
impl ::core::clone::Clone for ColorChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorPicker {}
impl ::core::clone::Clone for ColorPicker {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ColumnDefinition {}
impl ::core::clone::Clone for ColumnDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColumnDefinitionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColumnDefinitionCollection {}
impl ::core::clone::Clone for ColumnDefinitionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBox {}
impl ::core::clone::Clone for ComboBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ComboBoxItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ComboBoxItem {}
impl ::core::clone::Clone for ComboBoxItem {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ComboBoxTextSubmittedEventArgs {}
impl ::core::clone::Clone for ComboBoxTextSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommandBar {}
impl ::core::clone::Clone for CommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CommandBarFlyout {}
impl ::core::clone::Clone for CommandBarFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CommandBarOverflowPresenter {}
impl ::core::clone::Clone for CommandBarOverflowPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContainerContentChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContainerContentChangingEventArgs {}
impl ::core::clone::Clone for ContainerContentChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentControl {}
impl ::core::clone::Clone for ContentControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialog {}
impl ::core::clone::Clone for ContentDialog {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContentDialogButtonClickDeferral {}
impl ::core::clone::Clone for ContentDialogButtonClickDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogButtonClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialogButtonClickEventArgs {}
impl ::core::clone::Clone for ContentDialogButtonClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialogClosedEventArgs {}
impl ::core::clone::Clone for ContentDialogClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogClosingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialogClosingDeferral {}
impl ::core::clone::Clone for ContentDialogClosingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialogClosingEventArgs {}
impl ::core::clone::Clone for ContentDialogClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentDialogOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentDialogOpenedEventArgs {}
impl ::core::clone::Clone for ContentDialogOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ContentLinkChangedEventArgs {}
impl ::core::clone::Clone for ContentLinkChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentPresenter {}
impl ::core::clone::Clone for ContentPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContextMenuEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContextMenuEventArgs {}
impl ::core::clone::Clone for ContextMenuEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContextMenuOpeningEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContextMenuOpeningEventHandler {}
impl ::core::clone::Clone for ContextMenuOpeningEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Control(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Control {}
impl ::core::clone::Clone for Control {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ControlTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ControlTemplate {}
impl ::core::clone::Clone for ControlTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataTemplateSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataTemplateSelector {}
impl ::core::clone::Clone for DataTemplateSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickedEventArgs {}
impl ::core::clone::Clone for DatePickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePicker {}
impl ::core::clone::Clone for DatePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerFlyout {}
impl ::core::clone::Clone for DatePickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerFlyoutItem {}
impl ::core::clone::Clone for DatePickerFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerFlyoutPresenter {}
impl ::core::clone::Clone for DatePickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerSelectedValueChangedEventArgs {}
impl ::core::clone::Clone for DatePickerSelectedValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DatePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DatePickerValueChangedEventArgs {}
impl ::core::clone::Clone for DatePickerValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DragItemsCompletedEventArgs {}
impl ::core::clone::Clone for DragItemsCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragItemsStartingEventArgs {}
impl ::core::clone::Clone for DragItemsStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DragItemsStartingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DragItemsStartingEventHandler {}
impl ::core::clone::Clone for DragItemsStartingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DropDownButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DropDownButton {}
impl ::core::clone::Clone for DropDownButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DropDownButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DropDownButtonAutomationPeer {}
impl ::core::clone::Clone for DropDownButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DynamicOverflowItemsChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DynamicOverflowItemsChangingEventArgs {}
impl ::core::clone::Clone for DynamicOverflowItemsChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlipView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlipView {}
impl ::core::clone::Clone for FlipView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlipViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlipViewItem {}
impl ::core::clone::Clone for FlipViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Flyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Flyout {}
impl ::core::clone::Clone for Flyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FlyoutPresenter {}
impl ::core::clone::Clone for FlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusDisengagedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FocusDisengagedEventArgs {}
impl ::core::clone::Clone for FocusDisengagedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FocusEngagedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FocusEngagedEventArgs {}
impl ::core::clone::Clone for FocusEngagedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FontIcon {}
impl ::core::clone::Clone for FontIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FontIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FontIconSource {}
impl ::core::clone::Clone for FontIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Frame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Frame {}
impl ::core::clone::Clone for Frame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Grid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Grid {}
impl ::core::clone::Clone for Grid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridView {}
impl ::core::clone::Clone for GridView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewHeaderItem {}
impl ::core::clone::Clone for GridViewHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GridViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GridViewItem {}
impl ::core::clone::Clone for GridViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GroupItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GroupItem {}
impl ::core::clone::Clone for GroupItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GroupStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GroupStyle {}
impl ::core::clone::Clone for GroupStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GroupStyleSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GroupStyleSelector {}
impl ::core::clone::Clone for GroupStyleSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandwritingPanelClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandwritingPanelClosedEventArgs {}
impl ::core::clone::Clone for HandwritingPanelClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandwritingPanelOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandwritingPanelOpenedEventArgs {}
impl ::core::clone::Clone for HandwritingPanelOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for HandwritingView {}
impl ::core::clone::Clone for HandwritingView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandwritingViewCandidatesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandwritingViewCandidatesChangedEventArgs {}
impl ::core::clone::Clone for HandwritingViewCandidatesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HandwritingViewTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HandwritingViewTextSubmittedEventArgs {}
impl ::core::clone::Clone for HandwritingViewTextSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Hub(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Hub {}
impl ::core::clone::Clone for Hub {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubSection {}
impl ::core::clone::Clone for HubSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubSectionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubSectionCollection {}
impl ::core::clone::Clone for HubSectionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubSectionHeaderClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubSectionHeaderClickEventArgs {}
impl ::core::clone::Clone for HubSectionHeaderClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HubSectionHeaderClickEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HubSectionHeaderClickEventHandler {}
impl ::core::clone::Clone for HubSectionHeaderClickEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HyperlinkButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HyperlinkButton {}
impl ::core::clone::Clone for HyperlinkButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnchorRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnchorRequestedEventArgs {}
impl ::core::clone::Clone for IAnchorRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBar {}
impl ::core::clone::Clone for IAppBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBar2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBar2 {}
impl ::core::clone::Clone for IAppBar2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBar3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBar3 {}
impl ::core::clone::Clone for IAppBar3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBar4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBar4 {}
impl ::core::clone::Clone for IAppBar4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButton {}
impl ::core::clone::Clone for IAppBarButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButton3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButton3 {}
impl ::core::clone::Clone for IAppBarButton3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButton4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButton4 {}
impl ::core::clone::Clone for IAppBarButton4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButton5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButton5 {}
impl ::core::clone::Clone for IAppBarButton5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonFactory {}
impl ::core::clone::Clone for IAppBarButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonStatics {}
impl ::core::clone::Clone for IAppBarButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonStatics3 {}
impl ::core::clone::Clone for IAppBarButtonStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarButtonStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarButtonStatics4 {}
impl ::core::clone::Clone for IAppBarButtonStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarElementContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarElementContainer {}
impl ::core::clone::Clone for IAppBarElementContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarElementContainerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarElementContainerFactory {}
impl ::core::clone::Clone for IAppBarElementContainerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarElementContainerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarElementContainerStatics {}
impl ::core::clone::Clone for IAppBarElementContainerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarFactory {}
impl ::core::clone::Clone for IAppBarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarOverrides {}
impl ::core::clone::Clone for IAppBarOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarOverrides3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarOverrides3 {}
impl ::core::clone::Clone for IAppBarOverrides3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarSeparator {}
impl ::core::clone::Clone for IAppBarSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarSeparatorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarSeparatorFactory {}
impl ::core::clone::Clone for IAppBarSeparatorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarSeparatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarSeparatorStatics {}
impl ::core::clone::Clone for IAppBarSeparatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarSeparatorStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarSeparatorStatics3 {}
impl ::core::clone::Clone for IAppBarSeparatorStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarStatics {}
impl ::core::clone::Clone for IAppBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarStatics2 {}
impl ::core::clone::Clone for IAppBarStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarStatics4 {}
impl ::core::clone::Clone for IAppBarStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButton {}
impl ::core::clone::Clone for IAppBarToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButton3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButton3 {}
impl ::core::clone::Clone for IAppBarToggleButton3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButton4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButton4 {}
impl ::core::clone::Clone for IAppBarToggleButton4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButton5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButton5 {}
impl ::core::clone::Clone for IAppBarToggleButton5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonFactory {}
impl ::core::clone::Clone for IAppBarToggleButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonStatics {}
impl ::core::clone::Clone for IAppBarToggleButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonStatics3 {}
impl ::core::clone::Clone for IAppBarToggleButtonStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBarToggleButtonStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBarToggleButtonStatics4 {}
impl ::core::clone::Clone for IAppBarToggleButtonStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBox {}
impl ::core::clone::Clone for IAutoSuggestBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBox2 {}
impl ::core::clone::Clone for IAutoSuggestBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBox3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBox3 {}
impl ::core::clone::Clone for IAutoSuggestBox3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBox4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBox4 {}
impl ::core::clone::Clone for IAutoSuggestBox4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxQuerySubmittedEventArgs {}
impl ::core::clone::Clone for IAutoSuggestBoxQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxStatics {}
impl ::core::clone::Clone for IAutoSuggestBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxStatics2 {}
impl ::core::clone::Clone for IAutoSuggestBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxStatics3 {}
impl ::core::clone::Clone for IAutoSuggestBoxStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxStatics4 {}
impl ::core::clone::Clone for IAutoSuggestBoxStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxSuggestionChosenEventArgs {}
impl ::core::clone::Clone for IAutoSuggestBoxSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxTextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxTextChangedEventArgs {}
impl ::core::clone::Clone for IAutoSuggestBoxTextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutoSuggestBoxTextChangedEventArgsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutoSuggestBoxTextChangedEventArgsStatics {}
impl ::core::clone::Clone for IAutoSuggestBoxTextChangedEventArgsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackClickEventArgs {}
impl ::core::clone::Clone for IBackClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIcon {}
impl ::core::clone::Clone for IBitmapIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIcon2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIcon2 {}
impl ::core::clone::Clone for IBitmapIcon2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconFactory {}
impl ::core::clone::Clone for IBitmapIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconSource {}
impl ::core::clone::Clone for IBitmapIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconSourceFactory {}
impl ::core::clone::Clone for IBitmapIconSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconSourceStatics {}
impl ::core::clone::Clone for IBitmapIconSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconStatics {}
impl ::core::clone::Clone for IBitmapIconStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBitmapIconStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBitmapIconStatics2 {}
impl ::core::clone::Clone for IBitmapIconStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBorder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBorder {}
impl ::core::clone::Clone for IBorder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBorder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBorder2 {}
impl ::core::clone::Clone for IBorder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBorderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBorderStatics {}
impl ::core::clone::Clone for IBorderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBorderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBorderStatics2 {}
impl ::core::clone::Clone for IBorderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButton {}
impl ::core::clone::Clone for IButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonFactory {}
impl ::core::clone::Clone for IButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonStaticsWithFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonStaticsWithFlyout {}
impl ::core::clone::Clone for IButtonStaticsWithFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IButtonWithFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IButtonWithFlyout {}
impl ::core::clone::Clone for IButtonWithFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePicker {}
impl ::core::clone::Clone for ICalendarDatePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePicker2 {}
impl ::core::clone::Clone for ICalendarDatePicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePicker3 {}
impl ::core::clone::Clone for ICalendarDatePicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerDateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerDateChangedEventArgs {}
impl ::core::clone::Clone for ICalendarDatePickerDateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerFactory {}
impl ::core::clone::Clone for ICalendarDatePickerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerStatics {}
impl ::core::clone::Clone for ICalendarDatePickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerStatics2 {}
impl ::core::clone::Clone for ICalendarDatePickerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarDatePickerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarDatePickerStatics3 {}
impl ::core::clone::Clone for ICalendarDatePickerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarView {}
impl ::core::clone::Clone for ICalendarView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarView2 {}
impl ::core::clone::Clone for ICalendarView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewDayItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewDayItem {}
impl ::core::clone::Clone for ICalendarViewDayItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewDayItemChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewDayItemChangingEventArgs {}
impl ::core::clone::Clone for ICalendarViewDayItemChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewDayItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewDayItemFactory {}
impl ::core::clone::Clone for ICalendarViewDayItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewDayItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewDayItemStatics {}
impl ::core::clone::Clone for ICalendarViewDayItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewFactory {}
impl ::core::clone::Clone for ICalendarViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewSelectedDatesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewSelectedDatesChangedEventArgs {}
impl ::core::clone::Clone for ICalendarViewSelectedDatesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewStatics {}
impl ::core::clone::Clone for ICalendarViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICalendarViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICalendarViewStatics2 {}
impl ::core::clone::Clone for ICalendarViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICandidateWindowBoundsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICandidateWindowBoundsChangedEventArgs {}
impl ::core::clone::Clone for ICandidateWindowBoundsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICanvas(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICanvas {}
impl ::core::clone::Clone for ICanvas {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICanvasFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICanvasFactory {}
impl ::core::clone::Clone for ICanvasFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICanvasStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICanvasStatics {}
impl ::core::clone::Clone for ICanvasStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICaptureElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICaptureElement {}
impl ::core::clone::Clone for ICaptureElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICaptureElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICaptureElementStatics {}
impl ::core::clone::Clone for ICaptureElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICheckBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICheckBox {}
impl ::core::clone::Clone for ICheckBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICheckBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICheckBoxFactory {}
impl ::core::clone::Clone for ICheckBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChoosingGroupHeaderContainerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChoosingGroupHeaderContainerEventArgs {}
impl ::core::clone::Clone for IChoosingGroupHeaderContainerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChoosingItemContainerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChoosingItemContainerEventArgs {}
impl ::core::clone::Clone for IChoosingItemContainerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICleanUpVirtualizedItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICleanUpVirtualizedItemEventArgs {}
impl ::core::clone::Clone for ICleanUpVirtualizedItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorChangedEventArgs {}
impl ::core::clone::Clone for IColorChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPicker {}
impl ::core::clone::Clone for IColorPicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerFactory {}
impl ::core::clone::Clone for IColorPickerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorPickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorPickerStatics {}
impl ::core::clone::Clone for IColorPickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColumnDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColumnDefinition {}
impl ::core::clone::Clone for IColumnDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColumnDefinitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColumnDefinitionStatics {}
impl ::core::clone::Clone for IColumnDefinitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox {}
impl ::core::clone::Clone for IComboBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox2 {}
impl ::core::clone::Clone for IComboBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox3 {}
impl ::core::clone::Clone for IComboBox3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox4 {}
impl ::core::clone::Clone for IComboBox4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox5 {}
impl ::core::clone::Clone for IComboBox5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBox6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBox6 {}
impl ::core::clone::Clone for IComboBox6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxFactory {}
impl ::core::clone::Clone for IComboBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItem {}
impl ::core::clone::Clone for IComboBoxItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxItemFactory {}
impl ::core::clone::Clone for IComboBoxItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxOverrides {}
impl ::core::clone::Clone for IComboBoxOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics {}
impl ::core::clone::Clone for IComboBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics2 {}
impl ::core::clone::Clone for IComboBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics3 {}
impl ::core::clone::Clone for IComboBoxStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics4 {}
impl ::core::clone::Clone for IComboBoxStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics5 {}
impl ::core::clone::Clone for IComboBoxStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxStatics6 {}
impl ::core::clone::Clone for IComboBoxStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComboBoxTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComboBoxTextSubmittedEventArgs {}
impl ::core::clone::Clone for IComboBoxTextSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBar {}
impl ::core::clone::Clone for ICommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBar2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBar2 {}
impl ::core::clone::Clone for ICommandBar2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBar3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBar3 {}
impl ::core::clone::Clone for ICommandBar3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarElement {}
impl ::core::clone::Clone for ICommandBarElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarElement2 {}
impl ::core::clone::Clone for ICommandBarElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFactory {}
impl ::core::clone::Clone for ICommandBarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFlyout {}
impl ::core::clone::Clone for ICommandBarFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarFlyoutFactory {}
impl ::core::clone::Clone for ICommandBarFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarOverflowPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarOverflowPresenter {}
impl ::core::clone::Clone for ICommandBarOverflowPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarOverflowPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarOverflowPresenterFactory {}
impl ::core::clone::Clone for ICommandBarOverflowPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarStatics {}
impl ::core::clone::Clone for ICommandBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarStatics2 {}
impl ::core::clone::Clone for ICommandBarStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommandBarStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommandBarStatics3 {}
impl ::core::clone::Clone for ICommandBarStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContainerContentChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContainerContentChangingEventArgs {}
impl ::core::clone::Clone for IContainerContentChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentControl {}
impl ::core::clone::Clone for IContentControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentControl2 {}
impl ::core::clone::Clone for IContentControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentControlFactory {}
impl ::core::clone::Clone for IContentControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentControlOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentControlOverrides {}
impl ::core::clone::Clone for IContentControlOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentControlStatics {}
impl ::core::clone::Clone for IContentControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialog {}
impl ::core::clone::Clone for IContentDialog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialog2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialog2 {}
impl ::core::clone::Clone for IContentDialog2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialog3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialog3 {}
impl ::core::clone::Clone for IContentDialog3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogButtonClickDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogButtonClickDeferral {}
impl ::core::clone::Clone for IContentDialogButtonClickDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogButtonClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogButtonClickEventArgs {}
impl ::core::clone::Clone for IContentDialogButtonClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogClosedEventArgs {}
impl ::core::clone::Clone for IContentDialogClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogClosingDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogClosingDeferral {}
impl ::core::clone::Clone for IContentDialogClosingDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogClosingEventArgs {}
impl ::core::clone::Clone for IContentDialogClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogFactory {}
impl ::core::clone::Clone for IContentDialogFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogOpenedEventArgs {}
impl ::core::clone::Clone for IContentDialogOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogStatics {}
impl ::core::clone::Clone for IContentDialogStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentDialogStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentDialogStatics2 {}
impl ::core::clone::Clone for IContentDialogStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentLinkChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentLinkChangedEventArgs {}
impl ::core::clone::Clone for IContentLinkChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenter {}
impl ::core::clone::Clone for IContentPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenter2 {}
impl ::core::clone::Clone for IContentPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenter3 {}
impl ::core::clone::Clone for IContentPresenter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenter4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenter4 {}
impl ::core::clone::Clone for IContentPresenter4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenter5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenter5 {}
impl ::core::clone::Clone for IContentPresenter5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterFactory {}
impl ::core::clone::Clone for IContentPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterOverrides {}
impl ::core::clone::Clone for IContentPresenterOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterStatics {}
impl ::core::clone::Clone for IContentPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterStatics2 {}
impl ::core::clone::Clone for IContentPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterStatics3 {}
impl ::core::clone::Clone for IContentPresenterStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterStatics4 {}
impl ::core::clone::Clone for IContentPresenterStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPresenterStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPresenterStatics5 {}
impl ::core::clone::Clone for IContentPresenterStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContextMenuEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContextMenuEventArgs {}
impl ::core::clone::Clone for IContextMenuEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl {}
impl ::core::clone::Clone for IControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl2 {}
impl ::core::clone::Clone for IControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl3 {}
impl ::core::clone::Clone for IControl3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl4 {}
impl ::core::clone::Clone for IControl4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl5 {}
impl ::core::clone::Clone for IControl5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControl7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControl7 {}
impl ::core::clone::Clone for IControl7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlFactory {}
impl ::core::clone::Clone for IControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlOverrides {}
impl ::core::clone::Clone for IControlOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlOverrides6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlOverrides6 {}
impl ::core::clone::Clone for IControlOverrides6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlProtected {}
impl ::core::clone::Clone for IControlProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics {}
impl ::core::clone::Clone for IControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics2 {}
impl ::core::clone::Clone for IControlStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics3 {}
impl ::core::clone::Clone for IControlStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics4 {}
impl ::core::clone::Clone for IControlStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics5 {}
impl ::core::clone::Clone for IControlStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlStatics7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlStatics7 {}
impl ::core::clone::Clone for IControlStatics7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IControlTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IControlTemplate {}
impl ::core::clone::Clone for IControlTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateSelector {}
impl ::core::clone::Clone for IDataTemplateSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateSelector2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateSelector2 {}
impl ::core::clone::Clone for IDataTemplateSelector2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateSelectorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateSelectorFactory {}
impl ::core::clone::Clone for IDataTemplateSelectorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateSelectorOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateSelectorOverrides {}
impl ::core::clone::Clone for IDataTemplateSelectorOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTemplateSelectorOverrides2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTemplateSelectorOverrides2 {}
impl ::core::clone::Clone for IDataTemplateSelectorOverrides2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickedEventArgs {}
impl ::core::clone::Clone for IDatePickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePicker {}
impl ::core::clone::Clone for IDatePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePicker2 {}
impl ::core::clone::Clone for IDatePicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePicker3 {}
impl ::core::clone::Clone for IDatePicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFactory {}
impl ::core::clone::Clone for IDatePickerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyout {}
impl ::core::clone::Clone for IDatePickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyout2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyout2 {}
impl ::core::clone::Clone for IDatePickerFlyout2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutItem {}
impl ::core::clone::Clone for IDatePickerFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutItemStatics {}
impl ::core::clone::Clone for IDatePickerFlyoutItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutPresenter {}
impl ::core::clone::Clone for IDatePickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutPresenter2 {}
impl ::core::clone::Clone for IDatePickerFlyoutPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutPresenterStatics2 {}
impl ::core::clone::Clone for IDatePickerFlyoutPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutStatics {}
impl ::core::clone::Clone for IDatePickerFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerFlyoutStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerFlyoutStatics2 {}
impl ::core::clone::Clone for IDatePickerFlyoutStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerSelectedValueChangedEventArgs {}
impl ::core::clone::Clone for IDatePickerSelectedValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerStatics {}
impl ::core::clone::Clone for IDatePickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerStatics2 {}
impl ::core::clone::Clone for IDatePickerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerStatics3 {}
impl ::core::clone::Clone for IDatePickerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDatePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDatePickerValueChangedEventArgs {}
impl ::core::clone::Clone for IDatePickerValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragItemsCompletedEventArgs {}
impl ::core::clone::Clone for IDragItemsCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDragItemsStartingEventArgs {}
impl ::core::clone::Clone for IDragItemsStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropDownButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropDownButton {}
impl ::core::clone::Clone for IDropDownButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropDownButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropDownButtonAutomationPeer {}
impl ::core::clone::Clone for IDropDownButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropDownButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropDownButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IDropDownButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDropDownButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDropDownButtonFactory {}
impl ::core::clone::Clone for IDropDownButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDynamicOverflowItemsChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDynamicOverflowItemsChangingEventArgs {}
impl ::core::clone::Clone for IDynamicOverflowItemsChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipView {}
impl ::core::clone::Clone for IFlipView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipView2 {}
impl ::core::clone::Clone for IFlipView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewFactory {}
impl ::core::clone::Clone for IFlipViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItem {}
impl ::core::clone::Clone for IFlipViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewItemFactory {}
impl ::core::clone::Clone for IFlipViewItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlipViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlipViewStatics2 {}
impl ::core::clone::Clone for IFlipViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyout {}
impl ::core::clone::Clone for IFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutFactory {}
impl ::core::clone::Clone for IFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenter {}
impl ::core::clone::Clone for IFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenter2 {}
impl ::core::clone::Clone for IFlyoutPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenterFactory {}
impl ::core::clone::Clone for IFlyoutPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutPresenterStatics2 {}
impl ::core::clone::Clone for IFlyoutPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFlyoutStatics {}
impl ::core::clone::Clone for IFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusDisengagedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusDisengagedEventArgs {}
impl ::core::clone::Clone for IFocusDisengagedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusEngagedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusEngagedEventArgs {}
impl ::core::clone::Clone for IFocusEngagedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFocusEngagedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFocusEngagedEventArgs2 {}
impl ::core::clone::Clone for IFocusEngagedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIcon {}
impl ::core::clone::Clone for IFontIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIcon2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIcon2 {}
impl ::core::clone::Clone for IFontIcon2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIcon3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIcon3 {}
impl ::core::clone::Clone for IFontIcon3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconFactory {}
impl ::core::clone::Clone for IFontIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconSource {}
impl ::core::clone::Clone for IFontIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconSourceFactory {}
impl ::core::clone::Clone for IFontIconSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconSourceStatics {}
impl ::core::clone::Clone for IFontIconSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconStatics {}
impl ::core::clone::Clone for IFontIconStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconStatics2 {}
impl ::core::clone::Clone for IFontIconStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFontIconStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFontIconStatics3 {}
impl ::core::clone::Clone for IFontIconStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrame {}
impl ::core::clone::Clone for IFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrame2 {}
impl ::core::clone::Clone for IFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrame3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrame3 {}
impl ::core::clone::Clone for IFrame3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrame4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrame4 {}
impl ::core::clone::Clone for IFrame4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrame5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrame5 {}
impl ::core::clone::Clone for IFrame5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameFactory {}
impl ::core::clone::Clone for IFrameFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameStatics {}
impl ::core::clone::Clone for IFrameStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameStatics2 {}
impl ::core::clone::Clone for IFrameStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFrameStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFrameStatics5 {}
impl ::core::clone::Clone for IFrameStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGrid {}
impl ::core::clone::Clone for IGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGrid2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGrid2 {}
impl ::core::clone::Clone for IGrid2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGrid3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGrid3 {}
impl ::core::clone::Clone for IGrid3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGrid4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGrid4 {}
impl ::core::clone::Clone for IGrid4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridFactory {}
impl ::core::clone::Clone for IGridFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridStatics {}
impl ::core::clone::Clone for IGridStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridStatics2 {}
impl ::core::clone::Clone for IGridStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridStatics3 {}
impl ::core::clone::Clone for IGridStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridStatics4 {}
impl ::core::clone::Clone for IGridStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridView {}
impl ::core::clone::Clone for IGridView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewFactory {}
impl ::core::clone::Clone for IGridViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewHeaderItem {}
impl ::core::clone::Clone for IGridViewHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewHeaderItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewHeaderItemFactory {}
impl ::core::clone::Clone for IGridViewHeaderItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItem {}
impl ::core::clone::Clone for IGridViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGridViewItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGridViewItemFactory {}
impl ::core::clone::Clone for IGridViewItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupItem {}
impl ::core::clone::Clone for IGroupItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupItemFactory {}
impl ::core::clone::Clone for IGroupItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyle {}
impl ::core::clone::Clone for IGroupStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyle2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyle2 {}
impl ::core::clone::Clone for IGroupStyle2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyleFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyleFactory {}
impl ::core::clone::Clone for IGroupStyleFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyleSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyleSelector {}
impl ::core::clone::Clone for IGroupStyleSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyleSelectorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyleSelectorFactory {}
impl ::core::clone::Clone for IGroupStyleSelectorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGroupStyleSelectorOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGroupStyleSelectorOverrides {}
impl ::core::clone::Clone for IGroupStyleSelectorOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingPanelClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingPanelClosedEventArgs {}
impl ::core::clone::Clone for IHandwritingPanelClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingPanelOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingPanelOpenedEventArgs {}
impl ::core::clone::Clone for IHandwritingPanelOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingView {}
impl ::core::clone::Clone for IHandwritingView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingView2 {}
impl ::core::clone::Clone for IHandwritingView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingViewCandidatesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingViewCandidatesChangedEventArgs {}
impl ::core::clone::Clone for IHandwritingViewCandidatesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingViewFactory {}
impl ::core::clone::Clone for IHandwritingViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingViewStatics {}
impl ::core::clone::Clone for IHandwritingViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingViewStatics2 {}
impl ::core::clone::Clone for IHandwritingViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwritingViewTextSubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwritingViewTextSubmittedEventArgs {}
impl ::core::clone::Clone for IHandwritingViewTextSubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHub(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHub {}
impl ::core::clone::Clone for IHub {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubFactory {}
impl ::core::clone::Clone for IHubFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSection {}
impl ::core::clone::Clone for IHubSection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSectionFactory {}
impl ::core::clone::Clone for IHubSectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSectionHeaderClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSectionHeaderClickEventArgs {}
impl ::core::clone::Clone for IHubSectionHeaderClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubSectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubSectionStatics {}
impl ::core::clone::Clone for IHubSectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHubStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHubStatics {}
impl ::core::clone::Clone for IHubStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkButton {}
impl ::core::clone::Clone for IHyperlinkButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkButtonFactory {}
impl ::core::clone::Clone for IHyperlinkButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHyperlinkButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHyperlinkButtonStatics {}
impl ::core::clone::Clone for IHyperlinkButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconElement {}
impl ::core::clone::Clone for IIconElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconElementFactory {}
impl ::core::clone::Clone for IIconElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconElementStatics {}
impl ::core::clone::Clone for IIconElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSource {}
impl ::core::clone::Clone for IIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSourceElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSourceElement {}
impl ::core::clone::Clone for IIconSourceElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSourceElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSourceElementFactory {}
impl ::core::clone::Clone for IIconSourceElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSourceElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSourceElementStatics {}
impl ::core::clone::Clone for IIconSourceElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSourceFactory {}
impl ::core::clone::Clone for IIconSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIconSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIconSourceStatics {}
impl ::core::clone::Clone for IIconSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImage {}
impl ::core::clone::Clone for IImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImage2 {}
impl ::core::clone::Clone for IImage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImage3 {}
impl ::core::clone::Clone for IImage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IImageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IImageStatics {}
impl ::core::clone::Clone for IImageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCanvas(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCanvas {}
impl ::core::clone::Clone for IInkCanvas {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCanvasFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCanvasFactory {}
impl ::core::clone::Clone for IInkCanvasFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbar {}
impl ::core::clone::Clone for IInkToolbar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbar2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbar2 {}
impl ::core::clone::Clone for IInkToolbar2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbar3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbar3 {}
impl ::core::clone::Clone for IInkToolbar3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarBallpointPenButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarBallpointPenButton {}
impl ::core::clone::Clone for IInkToolbarBallpointPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarBallpointPenButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarBallpointPenButtonFactory {}
impl ::core::clone::Clone for IInkToolbarBallpointPenButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPen(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPen {}
impl ::core::clone::Clone for IInkToolbarCustomPen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPenButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPenButton {}
impl ::core::clone::Clone for IInkToolbarCustomPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPenButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPenButtonFactory {}
impl ::core::clone::Clone for IInkToolbarCustomPenButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPenButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPenButtonStatics {}
impl ::core::clone::Clone for IInkToolbarCustomPenButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPenFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPenFactory {}
impl ::core::clone::Clone for IInkToolbarCustomPenFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomPenOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomPenOverrides {}
impl ::core::clone::Clone for IInkToolbarCustomPenOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomToggleButton {}
impl ::core::clone::Clone for IInkToolbarCustomToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomToggleButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomToggleButtonFactory {}
impl ::core::clone::Clone for IInkToolbarCustomToggleButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomToolButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomToolButton {}
impl ::core::clone::Clone for IInkToolbarCustomToolButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomToolButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomToolButtonFactory {}
impl ::core::clone::Clone for IInkToolbarCustomToolButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarCustomToolButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarCustomToolButtonStatics {}
impl ::core::clone::Clone for IInkToolbarCustomToolButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarEraserButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarEraserButton {}
impl ::core::clone::Clone for IInkToolbarEraserButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarEraserButton2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarEraserButton2 {}
impl ::core::clone::Clone for IInkToolbarEraserButton2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarEraserButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarEraserButtonFactory {}
impl ::core::clone::Clone for IInkToolbarEraserButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarEraserButtonStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarEraserButtonStatics2 {}
impl ::core::clone::Clone for IInkToolbarEraserButtonStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarFactory {}
impl ::core::clone::Clone for IInkToolbarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarFlyoutItem {}
impl ::core::clone::Clone for IInkToolbarFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarFlyoutItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarFlyoutItemFactory {}
impl ::core::clone::Clone for IInkToolbarFlyoutItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarFlyoutItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarFlyoutItemStatics {}
impl ::core::clone::Clone for IInkToolbarFlyoutItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarHighlighterButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarHighlighterButton {}
impl ::core::clone::Clone for IInkToolbarHighlighterButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarHighlighterButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarHighlighterButtonFactory {}
impl ::core::clone::Clone for IInkToolbarHighlighterButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarIsStencilButtonCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarIsStencilButtonCheckedChangedEventArgs {}
impl ::core::clone::Clone for IInkToolbarIsStencilButtonCheckedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarMenuButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarMenuButton {}
impl ::core::clone::Clone for IInkToolbarMenuButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarMenuButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarMenuButtonFactory {}
impl ::core::clone::Clone for IInkToolbarMenuButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarMenuButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarMenuButtonStatics {}
impl ::core::clone::Clone for IInkToolbarMenuButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenButton {}
impl ::core::clone::Clone for IInkToolbarPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenButtonFactory {}
impl ::core::clone::Clone for IInkToolbarPenButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenButtonStatics {}
impl ::core::clone::Clone for IInkToolbarPenButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenConfigurationControl {}
impl ::core::clone::Clone for IInkToolbarPenConfigurationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenConfigurationControlFactory {}
impl ::core::clone::Clone for IInkToolbarPenConfigurationControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPenConfigurationControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPenConfigurationControlStatics {}
impl ::core::clone::Clone for IInkToolbarPenConfigurationControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPencilButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPencilButton {}
impl ::core::clone::Clone for IInkToolbarPencilButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarPencilButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarPencilButtonFactory {}
impl ::core::clone::Clone for IInkToolbarPencilButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarRulerButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarRulerButton {}
impl ::core::clone::Clone for IInkToolbarRulerButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarRulerButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarRulerButtonFactory {}
impl ::core::clone::Clone for IInkToolbarRulerButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarRulerButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarRulerButtonStatics {}
impl ::core::clone::Clone for IInkToolbarRulerButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStatics {}
impl ::core::clone::Clone for IInkToolbarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStatics2 {}
impl ::core::clone::Clone for IInkToolbarStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStatics3 {}
impl ::core::clone::Clone for IInkToolbarStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStencilButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStencilButton {}
impl ::core::clone::Clone for IInkToolbarStencilButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStencilButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStencilButtonFactory {}
impl ::core::clone::Clone for IInkToolbarStencilButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarStencilButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarStencilButtonStatics {}
impl ::core::clone::Clone for IInkToolbarStencilButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarToggleButton {}
impl ::core::clone::Clone for IInkToolbarToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarToggleButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarToggleButtonFactory {}
impl ::core::clone::Clone for IInkToolbarToggleButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarToolButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarToolButton {}
impl ::core::clone::Clone for IInkToolbarToolButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarToolButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarToolButtonFactory {}
impl ::core::clone::Clone for IInkToolbarToolButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkToolbarToolButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkToolbarToolButtonStatics {}
impl ::core::clone::Clone for IInkToolbarToolButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInsertionPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInsertionPanel {}
impl ::core::clone::Clone for IInsertionPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIsTextTrimmedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIsTextTrimmedChangedEventArgs {}
impl ::core::clone::Clone for IIsTextTrimmedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemClickEventArgs {}
impl ::core::clone::Clone for IItemClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemContainerGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemContainerGenerator {}
impl ::core::clone::Clone for IItemContainerGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemContainerMapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemContainerMapping {}
impl ::core::clone::Clone for IItemContainerMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControl {}
impl ::core::clone::Clone for IItemsControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControl2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControl2 {}
impl ::core::clone::Clone for IItemsControl2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControl3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControl3 {}
impl ::core::clone::Clone for IItemsControl3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlFactory {}
impl ::core::clone::Clone for IItemsControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlOverrides {}
impl ::core::clone::Clone for IItemsControlOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsControlStatics {}
impl ::core::clone::Clone for IItemsControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPanelTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPanelTemplate {}
impl ::core::clone::Clone for IItemsPanelTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPickedEventArgs {}
impl ::core::clone::Clone for IItemsPickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPresenter {}
impl ::core::clone::Clone for IItemsPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPresenter2 {}
impl ::core::clone::Clone for IItemsPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPresenterStatics {}
impl ::core::clone::Clone for IItemsPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsPresenterStatics2 {}
impl ::core::clone::Clone for IItemsPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsStackPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsStackPanel {}
impl ::core::clone::Clone for IItemsStackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsStackPanel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsStackPanel2 {}
impl ::core::clone::Clone for IItemsStackPanel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsStackPanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsStackPanelStatics {}
impl ::core::clone::Clone for IItemsStackPanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsStackPanelStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsStackPanelStatics2 {}
impl ::core::clone::Clone for IItemsStackPanelStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsWrapGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsWrapGrid {}
impl ::core::clone::Clone for IItemsWrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsWrapGrid2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsWrapGrid2 {}
impl ::core::clone::Clone for IItemsWrapGrid2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsWrapGridStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsWrapGridStatics {}
impl ::core::clone::Clone for IItemsWrapGridStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsWrapGridStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsWrapGridStatics2 {}
impl ::core::clone::Clone for IItemsWrapGridStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBox {}
impl ::core::clone::Clone for IListBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBox2 {}
impl ::core::clone::Clone for IListBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxFactory {}
impl ::core::clone::Clone for IListBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItem {}
impl ::core::clone::Clone for IListBoxItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxItemFactory {}
impl ::core::clone::Clone for IListBoxItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxStatics {}
impl ::core::clone::Clone for IListBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListBoxStatics2 {}
impl ::core::clone::Clone for IListBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListPickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListPickerFlyout {}
impl ::core::clone::Clone for IListPickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListPickerFlyoutPresenter {}
impl ::core::clone::Clone for IListPickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListPickerFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListPickerFlyoutStatics {}
impl ::core::clone::Clone for IListPickerFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListView {}
impl ::core::clone::Clone for IListView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase {}
impl ::core::clone::Clone for IListViewBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase2 {}
impl ::core::clone::Clone for IListViewBase2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase3 {}
impl ::core::clone::Clone for IListViewBase3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase4 {}
impl ::core::clone::Clone for IListViewBase4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase5 {}
impl ::core::clone::Clone for IListViewBase5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBase6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBase6 {}
impl ::core::clone::Clone for IListViewBase6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseFactory {}
impl ::core::clone::Clone for IListViewBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseHeaderItem {}
impl ::core::clone::Clone for IListViewBaseHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseHeaderItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseHeaderItemFactory {}
impl ::core::clone::Clone for IListViewBaseHeaderItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseStatics {}
impl ::core::clone::Clone for IListViewBaseStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseStatics2 {}
impl ::core::clone::Clone for IListViewBaseStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseStatics3 {}
impl ::core::clone::Clone for IListViewBaseStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseStatics4 {}
impl ::core::clone::Clone for IListViewBaseStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewBaseStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewBaseStatics5 {}
impl ::core::clone::Clone for IListViewBaseStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewFactory {}
impl ::core::clone::Clone for IListViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewHeaderItem {}
impl ::core::clone::Clone for IListViewHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewHeaderItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewHeaderItemFactory {}
impl ::core::clone::Clone for IListViewHeaderItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItem {}
impl ::core::clone::Clone for IListViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewItemFactory {}
impl ::core::clone::Clone for IListViewItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewPersistenceHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewPersistenceHelper {}
impl ::core::clone::Clone for IListViewPersistenceHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IListViewPersistenceHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IListViewPersistenceHelperStatics {}
impl ::core::clone::Clone for IListViewPersistenceHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElement {}
impl ::core::clone::Clone for IMediaElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElement2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElement2 {}
impl ::core::clone::Clone for IMediaElement2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElement3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElement3 {}
impl ::core::clone::Clone for IMediaElement3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElementStatics {}
impl ::core::clone::Clone for IMediaElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaElementStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaElementStatics2 {}
impl ::core::clone::Clone for IMediaElementStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerElement {}
impl ::core::clone::Clone for IMediaPlayerElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerElementFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerElementFactory {}
impl ::core::clone::Clone for IMediaPlayerElementFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerElementStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerElementStatics {}
impl ::core::clone::Clone for IMediaPlayerElementStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerPresenter {}
impl ::core::clone::Clone for IMediaPlayerPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerPresenterFactory {}
impl ::core::clone::Clone for IMediaPlayerPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPlayerPresenterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPlayerPresenterStatics {}
impl ::core::clone::Clone for IMediaPlayerPresenterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControls {}
impl ::core::clone::Clone for IMediaTransportControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControls2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControls2 {}
impl ::core::clone::Clone for IMediaTransportControls2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControls3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControls3 {}
impl ::core::clone::Clone for IMediaTransportControls3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControls4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControls4 {}
impl ::core::clone::Clone for IMediaTransportControls4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsFactory {}
impl ::core::clone::Clone for IMediaTransportControlsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsHelper {}
impl ::core::clone::Clone for IMediaTransportControlsHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsHelperStatics {}
impl ::core::clone::Clone for IMediaTransportControlsHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsStatics {}
impl ::core::clone::Clone for IMediaTransportControlsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsStatics2 {}
impl ::core::clone::Clone for IMediaTransportControlsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsStatics3 {}
impl ::core::clone::Clone for IMediaTransportControlsStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaTransportControlsStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaTransportControlsStatics4 {}
impl ::core::clone::Clone for IMediaTransportControlsStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBar {}
impl ::core::clone::Clone for IMenuBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarFactory {}
impl ::core::clone::Clone for IMenuBarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItem {}
impl ::core::clone::Clone for IMenuBarItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemFactory {}
impl ::core::clone::Clone for IMenuBarItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemFlyout {}
impl ::core::clone::Clone for IMenuBarItemFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemFlyoutFactory {}
impl ::core::clone::Clone for IMenuBarItemFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarItemStatics {}
impl ::core::clone::Clone for IMenuBarItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuBarStatics {}
impl ::core::clone::Clone for IMenuBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyout {}
impl ::core::clone::Clone for IMenuFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyout2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyout2 {}
impl ::core::clone::Clone for IMenuFlyout2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutFactory {}
impl ::core::clone::Clone for IMenuFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItem {}
impl ::core::clone::Clone for IMenuFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItem2 {}
impl ::core::clone::Clone for IMenuFlyoutItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItem3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItem3 {}
impl ::core::clone::Clone for IMenuFlyoutItem3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemBase {}
impl ::core::clone::Clone for IMenuFlyoutItemBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemBaseFactory {}
impl ::core::clone::Clone for IMenuFlyoutItemBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemFactory {}
impl ::core::clone::Clone for IMenuFlyoutItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemStatics {}
impl ::core::clone::Clone for IMenuFlyoutItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemStatics2 {}
impl ::core::clone::Clone for IMenuFlyoutItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutItemStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutItemStatics3 {}
impl ::core::clone::Clone for IMenuFlyoutItemStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenter {}
impl ::core::clone::Clone for IMenuFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenter2 {}
impl ::core::clone::Clone for IMenuFlyoutPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenter3 {}
impl ::core::clone::Clone for IMenuFlyoutPresenter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenterFactory {}
impl ::core::clone::Clone for IMenuFlyoutPresenterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutPresenterStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutPresenterStatics3 {}
impl ::core::clone::Clone for IMenuFlyoutPresenterStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSeparator {}
impl ::core::clone::Clone for IMenuFlyoutSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSeparatorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSeparatorFactory {}
impl ::core::clone::Clone for IMenuFlyoutSeparatorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutStatics {}
impl ::core::clone::Clone for IMenuFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSubItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSubItem {}
impl ::core::clone::Clone for IMenuFlyoutSubItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSubItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSubItem2 {}
impl ::core::clone::Clone for IMenuFlyoutSubItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSubItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSubItemStatics {}
impl ::core::clone::Clone for IMenuFlyoutSubItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMenuFlyoutSubItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMenuFlyoutSubItemStatics2 {}
impl ::core::clone::Clone for IMenuFlyoutSubItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigate {}
impl ::core::clone::Clone for INavigate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationView {}
impl ::core::clone::Clone for INavigationView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationView2 {}
impl ::core::clone::Clone for INavigationView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationView3 {}
impl ::core::clone::Clone for INavigationView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewBackRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewBackRequestedEventArgs {}
impl ::core::clone::Clone for INavigationViewBackRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewDisplayModeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewDisplayModeChangedEventArgs {}
impl ::core::clone::Clone for INavigationViewDisplayModeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewFactory {}
impl ::core::clone::Clone for INavigationViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItem {}
impl ::core::clone::Clone for INavigationViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItem2 {}
impl ::core::clone::Clone for INavigationViewItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemBase {}
impl ::core::clone::Clone for INavigationViewItemBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemBaseFactory {}
impl ::core::clone::Clone for INavigationViewItemBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemFactory {}
impl ::core::clone::Clone for INavigationViewItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemHeader {}
impl ::core::clone::Clone for INavigationViewItemHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemHeaderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemHeaderFactory {}
impl ::core::clone::Clone for INavigationViewItemHeaderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemInvokedEventArgs {}
impl ::core::clone::Clone for INavigationViewItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemInvokedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemInvokedEventArgs2 {}
impl ::core::clone::Clone for INavigationViewItemInvokedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemSeparator {}
impl ::core::clone::Clone for INavigationViewItemSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemSeparatorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemSeparatorFactory {}
impl ::core::clone::Clone for INavigationViewItemSeparatorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemStatics {}
impl ::core::clone::Clone for INavigationViewItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewItemStatics2 {}
impl ::core::clone::Clone for INavigationViewItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewList {}
impl ::core::clone::Clone for INavigationViewList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewListFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewListFactory {}
impl ::core::clone::Clone for INavigationViewListFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewPaneClosingEventArgs {}
impl ::core::clone::Clone for INavigationViewPaneClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewSelectionChangedEventArgs {}
impl ::core::clone::Clone for INavigationViewSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewSelectionChangedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewSelectionChangedEventArgs2 {}
impl ::core::clone::Clone for INavigationViewSelectionChangedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewStatics {}
impl ::core::clone::Clone for INavigationViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewStatics2 {}
impl ::core::clone::Clone for INavigationViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewStatics3 {}
impl ::core::clone::Clone for INavigationViewStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewTemplateSettings {}
impl ::core::clone::Clone for INavigationViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewTemplateSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewTemplateSettingsFactory {}
impl ::core::clone::Clone for INavigationViewTemplateSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INavigationViewTemplateSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INavigationViewTemplateSettingsStatics {}
impl ::core::clone::Clone for INavigationViewTemplateSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyEventArgs {}
impl ::core::clone::Clone for INotifyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyEventArgs2 {}
impl ::core::clone::Clone for INotifyEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPage {}
impl ::core::clone::Clone for IPage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageFactory {}
impl ::core::clone::Clone for IPageFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageOverrides {}
impl ::core::clone::Clone for IPageOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPageStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPageStatics {}
impl ::core::clone::Clone for IPageStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPanel {}
impl ::core::clone::Clone for IPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPanel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPanel2 {}
impl ::core::clone::Clone for IPanel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPanelFactory {}
impl ::core::clone::Clone for IPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPanelStatics {}
impl ::core::clone::Clone for IPanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IParallaxView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IParallaxView {}
impl ::core::clone::Clone for IParallaxView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IParallaxViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IParallaxViewFactory {}
impl ::core::clone::Clone for IParallaxViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IParallaxViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IParallaxViewStatics {}
impl ::core::clone::Clone for IParallaxViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBox {}
impl ::core::clone::Clone for IPasswordBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBox2 {}
impl ::core::clone::Clone for IPasswordBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBox3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBox3 {}
impl ::core::clone::Clone for IPasswordBox3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBox4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBox4 {}
impl ::core::clone::Clone for IPasswordBox4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBox5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBox5 {}
impl ::core::clone::Clone for IPasswordBox5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxPasswordChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxPasswordChangingEventArgs {}
impl ::core::clone::Clone for IPasswordBoxPasswordChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxStatics {}
impl ::core::clone::Clone for IPasswordBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxStatics2 {}
impl ::core::clone::Clone for IPasswordBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxStatics3 {}
impl ::core::clone::Clone for IPasswordBoxStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPasswordBoxStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPasswordBoxStatics5 {}
impl ::core::clone::Clone for IPasswordBoxStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIcon {}
impl ::core::clone::Clone for IPathIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIconFactory {}
impl ::core::clone::Clone for IPathIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIconSource {}
impl ::core::clone::Clone for IPathIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIconSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIconSourceFactory {}
impl ::core::clone::Clone for IPathIconSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIconSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIconSourceStatics {}
impl ::core::clone::Clone for IPathIconSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPathIconStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPathIconStatics {}
impl ::core::clone::Clone for IPathIconStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersonPicture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersonPicture {}
impl ::core::clone::Clone for IPersonPicture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersonPictureFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersonPictureFactory {}
impl ::core::clone::Clone for IPersonPictureFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPersonPictureStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPersonPictureStatics {}
impl ::core::clone::Clone for IPersonPictureStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerConfirmedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerConfirmedEventArgs {}
impl ::core::clone::Clone for IPickerConfirmedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyout {}
impl ::core::clone::Clone for IPickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutPresenter {}
impl ::core::clone::Clone for IPickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPickerFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPickerFlyoutStatics {}
impl ::core::clone::Clone for IPickerFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivot {}
impl ::core::clone::Clone for IPivot {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivot2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivot2 {}
impl ::core::clone::Clone for IPivot2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivot3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivot3 {}
impl ::core::clone::Clone for IPivot3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotFactory {}
impl ::core::clone::Clone for IPivotFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItem {}
impl ::core::clone::Clone for IPivotItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemEventArgs {}
impl ::core::clone::Clone for IPivotItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemFactory {}
impl ::core::clone::Clone for IPivotItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotItemStatics {}
impl ::core::clone::Clone for IPivotItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotStatics {}
impl ::core::clone::Clone for IPivotStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotStatics2 {}
impl ::core::clone::Clone for IPivotStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPivotStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPivotStatics3 {}
impl ::core::clone::Clone for IPivotStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBar {}
impl ::core::clone::Clone for IProgressBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBarFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBarFactory {}
impl ::core::clone::Clone for IProgressBarFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressBarStatics {}
impl ::core::clone::Clone for IProgressBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressRing(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressRing {}
impl ::core::clone::Clone for IProgressRing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProgressRingStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProgressRingStatics {}
impl ::core::clone::Clone for IProgressRingStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadioButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadioButton {}
impl ::core::clone::Clone for IRadioButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadioButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadioButtonFactory {}
impl ::core::clone::Clone for IRadioButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadioButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadioButtonStatics {}
impl ::core::clone::Clone for IRadioButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingControl {}
impl ::core::clone::Clone for IRatingControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingControlFactory {}
impl ::core::clone::Clone for IRatingControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingControlStatics {}
impl ::core::clone::Clone for IRatingControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemFontInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemFontInfo {}
impl ::core::clone::Clone for IRatingItemFontInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemFontInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemFontInfoFactory {}
impl ::core::clone::Clone for IRatingItemFontInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemFontInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemFontInfoStatics {}
impl ::core::clone::Clone for IRatingItemFontInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemImageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemImageInfo {}
impl ::core::clone::Clone for IRatingItemImageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemImageInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemImageInfoFactory {}
impl ::core::clone::Clone for IRatingItemImageInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemImageInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemImageInfoStatics {}
impl ::core::clone::Clone for IRatingItemImageInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemInfo {}
impl ::core::clone::Clone for IRatingItemInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRatingItemInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRatingItemInfoFactory {}
impl ::core::clone::Clone for IRatingItemInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshContainer {}
impl ::core::clone::Clone for IRefreshContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshContainerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshContainerFactory {}
impl ::core::clone::Clone for IRefreshContainerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshContainerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshContainerStatics {}
impl ::core::clone::Clone for IRefreshContainerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshInteractionRatioChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshInteractionRatioChangedEventArgs {}
impl ::core::clone::Clone for IRefreshInteractionRatioChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshRequestedEventArgs {}
impl ::core::clone::Clone for IRefreshRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshStateChangedEventArgs {}
impl ::core::clone::Clone for IRefreshStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshVisualizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshVisualizer {}
impl ::core::clone::Clone for IRefreshVisualizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshVisualizerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshVisualizerFactory {}
impl ::core::clone::Clone for IRefreshVisualizerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRefreshVisualizerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRefreshVisualizerStatics {}
impl ::core::clone::Clone for IRefreshVisualizerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativePanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativePanel {}
impl ::core::clone::Clone for IRelativePanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativePanel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativePanel2 {}
impl ::core::clone::Clone for IRelativePanel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativePanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativePanelFactory {}
impl ::core::clone::Clone for IRelativePanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativePanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativePanelStatics {}
impl ::core::clone::Clone for IRelativePanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativePanelStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativePanelStatics2 {}
impl ::core::clone::Clone for IRelativePanelStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox {}
impl ::core::clone::Clone for IRichEditBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox2 {}
impl ::core::clone::Clone for IRichEditBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox3 {}
impl ::core::clone::Clone for IRichEditBox3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox4 {}
impl ::core::clone::Clone for IRichEditBox4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox5 {}
impl ::core::clone::Clone for IRichEditBox5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox6 {}
impl ::core::clone::Clone for IRichEditBox6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox7 {}
impl ::core::clone::Clone for IRichEditBox7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBox8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBox8 {}
impl ::core::clone::Clone for IRichEditBox8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxFactory {}
impl ::core::clone::Clone for IRichEditBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxSelectionChangingEventArgs {}
impl ::core::clone::Clone for IRichEditBoxSelectionChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics {}
impl ::core::clone::Clone for IRichEditBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics2 {}
impl ::core::clone::Clone for IRichEditBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics3 {}
impl ::core::clone::Clone for IRichEditBoxStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics4 {}
impl ::core::clone::Clone for IRichEditBoxStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics5 {}
impl ::core::clone::Clone for IRichEditBoxStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics6 {}
impl ::core::clone::Clone for IRichEditBoxStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics7 {}
impl ::core::clone::Clone for IRichEditBoxStatics7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxStatics8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxStatics8 {}
impl ::core::clone::Clone for IRichEditBoxStatics8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxTextChangingEventArgs {}
impl ::core::clone::Clone for IRichEditBoxTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichEditBoxTextChangingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichEditBoxTextChangingEventArgs2 {}
impl ::core::clone::Clone for IRichEditBoxTextChangingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock {}
impl ::core::clone::Clone for IRichTextBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock2 {}
impl ::core::clone::Clone for IRichTextBlock2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock3 {}
impl ::core::clone::Clone for IRichTextBlock3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock4 {}
impl ::core::clone::Clone for IRichTextBlock4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock5 {}
impl ::core::clone::Clone for IRichTextBlock5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlock6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlock6 {}
impl ::core::clone::Clone for IRichTextBlock6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflow {}
impl ::core::clone::Clone for IRichTextBlockOverflow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflow2 {}
impl ::core::clone::Clone for IRichTextBlockOverflow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflow3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflow3 {}
impl ::core::clone::Clone for IRichTextBlockOverflow3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflowStatics {}
impl ::core::clone::Clone for IRichTextBlockOverflowStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflowStatics2 {}
impl ::core::clone::Clone for IRichTextBlockOverflowStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockOverflowStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockOverflowStatics3 {}
impl ::core::clone::Clone for IRichTextBlockOverflowStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics {}
impl ::core::clone::Clone for IRichTextBlockStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics2 {}
impl ::core::clone::Clone for IRichTextBlockStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics3 {}
impl ::core::clone::Clone for IRichTextBlockStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics4 {}
impl ::core::clone::Clone for IRichTextBlockStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics5 {}
impl ::core::clone::Clone for IRichTextBlockStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRichTextBlockStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRichTextBlockStatics6 {}
impl ::core::clone::Clone for IRichTextBlockStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRowDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRowDefinition {}
impl ::core::clone::Clone for IRowDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRowDefinitionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRowDefinitionStatics {}
impl ::core::clone::Clone for IRowDefinitionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollAnchorProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollAnchorProvider {}
impl ::core::clone::Clone for IScrollAnchorProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollContentPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollContentPresenter {}
impl ::core::clone::Clone for IScrollContentPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollContentPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollContentPresenter2 {}
impl ::core::clone::Clone for IScrollContentPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollContentPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollContentPresenterStatics2 {}
impl ::core::clone::Clone for IScrollContentPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewer {}
impl ::core::clone::Clone for IScrollViewer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewer2 {}
impl ::core::clone::Clone for IScrollViewer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewer3 {}
impl ::core::clone::Clone for IScrollViewer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewer4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewer4 {}
impl ::core::clone::Clone for IScrollViewer4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerStatics {}
impl ::core::clone::Clone for IScrollViewerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerStatics2 {}
impl ::core::clone::Clone for IScrollViewerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerStatics4 {}
impl ::core::clone::Clone for IScrollViewerStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerView {}
impl ::core::clone::Clone for IScrollViewerView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerViewChangedEventArgs {}
impl ::core::clone::Clone for IScrollViewerViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScrollViewerViewChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScrollViewerViewChangingEventArgs {}
impl ::core::clone::Clone for IScrollViewerViewChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBox {}
impl ::core::clone::Clone for ISearchBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxFactory {}
impl ::core::clone::Clone for ISearchBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxQueryChangedEventArgs {}
impl ::core::clone::Clone for ISearchBoxQueryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxQuerySubmittedEventArgs {}
impl ::core::clone::Clone for ISearchBoxQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxResultSuggestionChosenEventArgs {}
impl ::core::clone::Clone for ISearchBoxResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxStatics {}
impl ::core::clone::Clone for ISearchBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISearchBoxSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISearchBoxSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for ISearchBoxSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISectionsInViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISectionsInViewChangedEventArgs {}
impl ::core::clone::Clone for ISectionsInViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISectionsInViewChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISectionsInViewChangedEventArgsFactory {}
impl ::core::clone::Clone for ISectionsInViewChangedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectionChangedEventArgs {}
impl ::core::clone::Clone for ISelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectionChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectionChangedEventArgsFactory {}
impl ::core::clone::Clone for ISelectionChangedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoom(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoom {}
impl ::core::clone::Clone for ISemanticZoom {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomInformation {}
impl ::core::clone::Clone for ISemanticZoomInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomLocation {}
impl ::core::clone::Clone for ISemanticZoomLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomStatics {}
impl ::core::clone::Clone for ISemanticZoomStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISemanticZoomViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISemanticZoomViewChangedEventArgs {}
impl ::core::clone::Clone for ISemanticZoomViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyout {}
impl ::core::clone::Clone for ISettingsFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyoutFactory {}
impl ::core::clone::Clone for ISettingsFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISettingsFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISettingsFlyoutStatics {}
impl ::core::clone::Clone for ISettingsFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlider {}
impl ::core::clone::Clone for ISlider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlider2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlider2 {}
impl ::core::clone::Clone for ISlider2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISliderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISliderFactory {}
impl ::core::clone::Clone for ISliderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISliderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISliderStatics {}
impl ::core::clone::Clone for ISliderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISliderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISliderStatics2 {}
impl ::core::clone::Clone for ISliderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButton {}
impl ::core::clone::Clone for ISplitButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButtonAutomationPeer {}
impl ::core::clone::Clone for ISplitButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButtonAutomationPeerFactory {}
impl ::core::clone::Clone for ISplitButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButtonClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButtonClickEventArgs {}
impl ::core::clone::Clone for ISplitButtonClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButtonFactory {}
impl ::core::clone::Clone for ISplitButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitButtonStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitButtonStatics {}
impl ::core::clone::Clone for ISplitButtonStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitView {}
impl ::core::clone::Clone for ISplitView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitView2 {}
impl ::core::clone::Clone for ISplitView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitView3 {}
impl ::core::clone::Clone for ISplitView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitViewFactory {}
impl ::core::clone::Clone for ISplitViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitViewPaneClosingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitViewPaneClosingEventArgs {}
impl ::core::clone::Clone for ISplitViewPaneClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitViewStatics {}
impl ::core::clone::Clone for ISplitViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISplitViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISplitViewStatics2 {}
impl ::core::clone::Clone for ISplitViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanel {}
impl ::core::clone::Clone for IStackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanel2 {}
impl ::core::clone::Clone for IStackPanel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanel4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanel4 {}
impl ::core::clone::Clone for IStackPanel4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanel5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanel5 {}
impl ::core::clone::Clone for IStackPanel5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanelFactory {}
impl ::core::clone::Clone for IStackPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanelStatics {}
impl ::core::clone::Clone for IStackPanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanelStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanelStatics2 {}
impl ::core::clone::Clone for IStackPanelStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanelStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanelStatics4 {}
impl ::core::clone::Clone for IStackPanelStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStackPanelStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStackPanelStatics5 {}
impl ::core::clone::Clone for IStackPanelStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStyleSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStyleSelector {}
impl ::core::clone::Clone for IStyleSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStyleSelectorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStyleSelectorFactory {}
impl ::core::clone::Clone for IStyleSelectorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStyleSelectorOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStyleSelectorOverrides {}
impl ::core::clone::Clone for IStyleSelectorOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainBackgroundPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainBackgroundPanel {}
impl ::core::clone::Clone for ISwapChainBackgroundPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainBackgroundPanel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainBackgroundPanel2 {}
impl ::core::clone::Clone for ISwapChainBackgroundPanel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainBackgroundPanelFactory {}
impl ::core::clone::Clone for ISwapChainBackgroundPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainPanel {}
impl ::core::clone::Clone for ISwapChainPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainPanelFactory {}
impl ::core::clone::Clone for ISwapChainPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainPanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainPanelStatics {}
impl ::core::clone::Clone for ISwapChainPanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeControl {}
impl ::core::clone::Clone for ISwipeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeControlFactory {}
impl ::core::clone::Clone for ISwipeControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeControlStatics {}
impl ::core::clone::Clone for ISwipeControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItem {}
impl ::core::clone::Clone for ISwipeItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItemFactory {}
impl ::core::clone::Clone for ISwipeItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItemInvokedEventArgs {}
impl ::core::clone::Clone for ISwipeItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItemStatics {}
impl ::core::clone::Clone for ISwipeItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItems {}
impl ::core::clone::Clone for ISwipeItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItemsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItemsFactory {}
impl ::core::clone::Clone for ISwipeItemsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwipeItemsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwipeItemsStatics {}
impl ::core::clone::Clone for ISwipeItemsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIcon {}
impl ::core::clone::Clone for ISymbolIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIconFactory {}
impl ::core::clone::Clone for ISymbolIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIconSource {}
impl ::core::clone::Clone for ISymbolIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIconSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIconSourceFactory {}
impl ::core::clone::Clone for ISymbolIconSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIconSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIconSourceStatics {}
impl ::core::clone::Clone for ISymbolIconSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISymbolIconStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISymbolIconStatics {}
impl ::core::clone::Clone for ISymbolIconStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock {}
impl ::core::clone::Clone for ITextBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock2 {}
impl ::core::clone::Clone for ITextBlock2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock3 {}
impl ::core::clone::Clone for ITextBlock3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock4 {}
impl ::core::clone::Clone for ITextBlock4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock5 {}
impl ::core::clone::Clone for ITextBlock5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock6 {}
impl ::core::clone::Clone for ITextBlock6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlock7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlock7 {}
impl ::core::clone::Clone for ITextBlock7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics {}
impl ::core::clone::Clone for ITextBlockStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics2 {}
impl ::core::clone::Clone for ITextBlockStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics3 {}
impl ::core::clone::Clone for ITextBlockStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics5 {}
impl ::core::clone::Clone for ITextBlockStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics6 {}
impl ::core::clone::Clone for ITextBlockStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBlockStatics7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBlockStatics7 {}
impl ::core::clone::Clone for ITextBlockStatics7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox {}
impl ::core::clone::Clone for ITextBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox2 {}
impl ::core::clone::Clone for ITextBox2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox3 {}
impl ::core::clone::Clone for ITextBox3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox4 {}
impl ::core::clone::Clone for ITextBox4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox5 {}
impl ::core::clone::Clone for ITextBox5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox6 {}
impl ::core::clone::Clone for ITextBox6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox7 {}
impl ::core::clone::Clone for ITextBox7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBox8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBox8 {}
impl ::core::clone::Clone for ITextBox8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxBeforeTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxBeforeTextChangingEventArgs {}
impl ::core::clone::Clone for ITextBoxBeforeTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxFactory {}
impl ::core::clone::Clone for ITextBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxSelectionChangingEventArgs {}
impl ::core::clone::Clone for ITextBoxSelectionChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics {}
impl ::core::clone::Clone for ITextBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics2 {}
impl ::core::clone::Clone for ITextBoxStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics3 {}
impl ::core::clone::Clone for ITextBoxStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics5 {}
impl ::core::clone::Clone for ITextBoxStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics6 {}
impl ::core::clone::Clone for ITextBoxStatics6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics7 {}
impl ::core::clone::Clone for ITextBoxStatics7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxStatics8(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxStatics8 {}
impl ::core::clone::Clone for ITextBoxStatics8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxTextChangingEventArgs {}
impl ::core::clone::Clone for ITextBoxTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextBoxTextChangingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextBoxTextChangingEventArgs2 {}
impl ::core::clone::Clone for ITextBoxTextChangingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextChangedEventArgs {}
impl ::core::clone::Clone for ITextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextCommandBarFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextCommandBarFlyout {}
impl ::core::clone::Clone for ITextCommandBarFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextCommandBarFlyoutFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextCommandBarFlyoutFactory {}
impl ::core::clone::Clone for ITextCommandBarFlyoutFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextCompositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextCompositionChangedEventArgs {}
impl ::core::clone::Clone for ITextCompositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextCompositionEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextCompositionEndedEventArgs {}
impl ::core::clone::Clone for ITextCompositionEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextCompositionStartedEventArgs {}
impl ::core::clone::Clone for ITextCompositionStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextControlCopyingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextControlCopyingToClipboardEventArgs {}
impl ::core::clone::Clone for ITextControlCopyingToClipboardEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextControlCuttingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextControlCuttingToClipboardEventArgs {}
impl ::core::clone::Clone for ITextControlCuttingToClipboardEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextControlPasteEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextControlPasteEventArgs {}
impl ::core::clone::Clone for ITextControlPasteEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickedEventArgs {}
impl ::core::clone::Clone for ITimePickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePicker {}
impl ::core::clone::Clone for ITimePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePicker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePicker2 {}
impl ::core::clone::Clone for ITimePicker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePicker3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePicker3 {}
impl ::core::clone::Clone for ITimePicker3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFactory {}
impl ::core::clone::Clone for ITimePickerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyout {}
impl ::core::clone::Clone for ITimePickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyoutPresenter {}
impl ::core::clone::Clone for ITimePickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyoutPresenter2 {}
impl ::core::clone::Clone for ITimePickerFlyoutPresenter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyoutPresenterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyoutPresenterStatics2 {}
impl ::core::clone::Clone for ITimePickerFlyoutPresenterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerFlyoutStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerFlyoutStatics {}
impl ::core::clone::Clone for ITimePickerFlyoutStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerSelectedValueChangedEventArgs {}
impl ::core::clone::Clone for ITimePickerSelectedValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerStatics {}
impl ::core::clone::Clone for ITimePickerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerStatics2 {}
impl ::core::clone::Clone for ITimePickerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerStatics3 {}
impl ::core::clone::Clone for ITimePickerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimePickerValueChangedEventArgs {}
impl ::core::clone::Clone for ITimePickerValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleMenuFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleMenuFlyoutItem {}
impl ::core::clone::Clone for IToggleMenuFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleMenuFlyoutItemFactory {}
impl ::core::clone::Clone for IToggleMenuFlyoutItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleMenuFlyoutItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleMenuFlyoutItemStatics {}
impl ::core::clone::Clone for IToggleMenuFlyoutItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSplitButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSplitButton {}
impl ::core::clone::Clone for IToggleSplitButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSplitButtonAutomationPeer {}
impl ::core::clone::Clone for IToggleSplitButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSplitButtonAutomationPeerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSplitButtonAutomationPeerFactory {}
impl ::core::clone::Clone for IToggleSplitButtonAutomationPeerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSplitButtonFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSplitButtonFactory {}
impl ::core::clone::Clone for IToggleSplitButtonFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSplitButtonIsCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSplitButtonIsCheckedChangedEventArgs {}
impl ::core::clone::Clone for IToggleSplitButtonIsCheckedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitch {}
impl ::core::clone::Clone for IToggleSwitch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitchOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitchOverrides {}
impl ::core::clone::Clone for IToggleSwitchOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToggleSwitchStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToggleSwitchStatics {}
impl ::core::clone::Clone for IToggleSwitchStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTip {}
impl ::core::clone::Clone for IToolTip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTip2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTip2 {}
impl ::core::clone::Clone for IToolTip2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipFactory {}
impl ::core::clone::Clone for IToolTipFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipService {}
impl ::core::clone::Clone for IToolTipService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipServiceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipServiceStatics {}
impl ::core::clone::Clone for IToolTipServiceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipStatics {}
impl ::core::clone::Clone for IToolTipStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToolTipStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToolTipStatics2 {}
impl ::core::clone::Clone for IToolTipStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeView {}
impl ::core::clone::Clone for ITreeView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeView2 {}
impl ::core::clone::Clone for ITreeView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewCollapsedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewCollapsedEventArgs {}
impl ::core::clone::Clone for ITreeViewCollapsedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewCollapsedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewCollapsedEventArgs2 {}
impl ::core::clone::Clone for ITreeViewCollapsedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewDragItemsCompletedEventArgs {}
impl ::core::clone::Clone for ITreeViewDragItemsCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewDragItemsStartingEventArgs {}
impl ::core::clone::Clone for ITreeViewDragItemsStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewExpandingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewExpandingEventArgs {}
impl ::core::clone::Clone for ITreeViewExpandingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewExpandingEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewExpandingEventArgs2 {}
impl ::core::clone::Clone for ITreeViewExpandingEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewFactory {}
impl ::core::clone::Clone for ITreeViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItem {}
impl ::core::clone::Clone for ITreeViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItem2 {}
impl ::core::clone::Clone for ITreeViewItem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemFactory {}
impl ::core::clone::Clone for ITreeViewItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemInvokedEventArgs {}
impl ::core::clone::Clone for ITreeViewItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemStatics {}
impl ::core::clone::Clone for ITreeViewItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemStatics2 {}
impl ::core::clone::Clone for ITreeViewItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemTemplateSettings {}
impl ::core::clone::Clone for ITreeViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettingsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemTemplateSettingsFactory {}
impl ::core::clone::Clone for ITreeViewItemTemplateSettingsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewItemTemplateSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewItemTemplateSettingsStatics {}
impl ::core::clone::Clone for ITreeViewItemTemplateSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewList {}
impl ::core::clone::Clone for ITreeViewList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewListFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewListFactory {}
impl ::core::clone::Clone for ITreeViewListFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewNode {}
impl ::core::clone::Clone for ITreeViewNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewNodeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewNodeFactory {}
impl ::core::clone::Clone for ITreeViewNodeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewNodeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewNodeStatics {}
impl ::core::clone::Clone for ITreeViewNodeStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewStatics {}
impl ::core::clone::Clone for ITreeViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITreeViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITreeViewStatics2 {}
impl ::core::clone::Clone for ITreeViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPaneView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPaneView {}
impl ::core::clone::Clone for ITwoPaneView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPaneViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPaneViewFactory {}
impl ::core::clone::Clone for ITwoPaneViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITwoPaneViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITwoPaneViewStatics {}
impl ::core::clone::Clone for ITwoPaneViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUIElementCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUIElementCollection {}
impl ::core::clone::Clone for IUIElementCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserControl {}
impl ::core::clone::Clone for IUserControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserControlFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserControlFactory {}
impl ::core::clone::Clone for IUserControlFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserControlStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserControlStatics {}
impl ::core::clone::Clone for IUserControlStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVariableSizedWrapGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariableSizedWrapGrid {}
impl ::core::clone::Clone for IVariableSizedWrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVariableSizedWrapGridStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariableSizedWrapGridStatics {}
impl ::core::clone::Clone for IVariableSizedWrapGridStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IViewbox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IViewbox {}
impl ::core::clone::Clone for IViewbox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IViewboxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IViewboxStatics {}
impl ::core::clone::Clone for IViewboxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingPanel {}
impl ::core::clone::Clone for IVirtualizingPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingPanelFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingPanelFactory {}
impl ::core::clone::Clone for IVirtualizingPanelFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingPanelOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingPanelOverrides {}
impl ::core::clone::Clone for IVirtualizingPanelOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingPanelProtected(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingPanelProtected {}
impl ::core::clone::Clone for IVirtualizingPanelProtected {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingStackPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingStackPanel {}
impl ::core::clone::Clone for IVirtualizingStackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingStackPanelOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingStackPanelOverrides {}
impl ::core::clone::Clone for IVirtualizingStackPanelOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualizingStackPanelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualizingStackPanelStatics {}
impl ::core::clone::Clone for IVirtualizingStackPanelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView {}
impl ::core::clone::Clone for IWebView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView2 {}
impl ::core::clone::Clone for IWebView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView3 {}
impl ::core::clone::Clone for IWebView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView4 {}
impl ::core::clone::Clone for IWebView4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView5 {}
impl ::core::clone::Clone for IWebView5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView6 {}
impl ::core::clone::Clone for IWebView6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebView7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebView7 {}
impl ::core::clone::Clone for IWebView7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewBrush {}
impl ::core::clone::Clone for IWebViewBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewBrushStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewBrushStatics {}
impl ::core::clone::Clone for IWebViewBrushStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewContentLoadingEventArgs {}
impl ::core::clone::Clone for IWebViewContentLoadingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewDOMContentLoadedEventArgs {}
impl ::core::clone::Clone for IWebViewDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewDeferredPermissionRequest {}
impl ::core::clone::Clone for IWebViewDeferredPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewFactory4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewFactory4 {}
impl ::core::clone::Clone for IWebViewFactory4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewLongRunningScriptDetectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewLongRunningScriptDetectedEventArgs {}
impl ::core::clone::Clone for IWebViewLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewNavigationCompletedEventArgs {}
impl ::core::clone::Clone for IWebViewNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewNavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewNavigationFailedEventArgs {}
impl ::core::clone::Clone for IWebViewNavigationFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewNavigationStartingEventArgs {}
impl ::core::clone::Clone for IWebViewNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewNewWindowRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewPermissionRequest {}
impl ::core::clone::Clone for IWebViewPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewPermissionRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewSeparateProcessLostEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewSeparateProcessLostEventArgs {}
impl ::core::clone::Clone for IWebViewSeparateProcessLostEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewSettings {}
impl ::core::clone::Clone for IWebViewSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewStatics {}
impl ::core::clone::Clone for IWebViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewStatics2 {}
impl ::core::clone::Clone for IWebViewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewStatics3 {}
impl ::core::clone::Clone for IWebViewStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewStatics4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewStatics4 {}
impl ::core::clone::Clone for IWebViewStatics4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewStatics5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewStatics5 {}
impl ::core::clone::Clone for IWebViewStatics5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::clone::Clone for IWebViewUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewUnviewableContentIdentifiedEventArgs {}
impl ::core::clone::Clone for IWebViewUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewUnviewableContentIdentifiedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewUnviewableContentIdentifiedEventArgs2 {}
impl ::core::clone::Clone for IWebViewUnviewableContentIdentifiedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebViewWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebViewWebResourceRequestedEventArgs {}
impl ::core::clone::Clone for IWebViewWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWrapGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWrapGrid {}
impl ::core::clone::Clone for IWrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWrapGridStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWrapGridStatics {}
impl ::core::clone::Clone for IWrapGridStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IconElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IconElement {}
impl ::core::clone::Clone for IconElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IconSource {}
impl ::core::clone::Clone for IconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IconSourceElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IconSourceElement {}
impl ::core::clone::Clone for IconSourceElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Image(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Image {}
impl ::core::clone::Clone for Image {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkCanvas {}
impl ::core::clone::Clone for InkCanvas {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbar {}
impl ::core::clone::Clone for InkToolbar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarBallpointPenButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarBallpointPenButton {}
impl ::core::clone::Clone for InkToolbarBallpointPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarCustomPen {}
impl ::core::clone::Clone for InkToolbarCustomPen {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarCustomPenButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarCustomPenButton {}
impl ::core::clone::Clone for InkToolbarCustomPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarCustomToggleButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarCustomToggleButton {}
impl ::core::clone::Clone for InkToolbarCustomToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarCustomToolButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarCustomToolButton {}
impl ::core::clone::Clone for InkToolbarCustomToolButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarEraserButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarEraserButton {}
impl ::core::clone::Clone for InkToolbarEraserButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarFlyoutItem {}
impl ::core::clone::Clone for InkToolbarFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarHighlighterButton {}
impl ::core::clone::Clone for InkToolbarHighlighterButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarIsStencilButtonCheckedChangedEventArgs {}
impl ::core::clone::Clone for InkToolbarIsStencilButtonCheckedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarMenuButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarMenuButton {}
impl ::core::clone::Clone for InkToolbarMenuButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarPenButton {}
impl ::core::clone::Clone for InkToolbarPenButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarPenConfigurationControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarPenConfigurationControl {}
impl ::core::clone::Clone for InkToolbarPenConfigurationControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarPencilButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarPencilButton {}
impl ::core::clone::Clone for InkToolbarPencilButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarRulerButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarRulerButton {}
impl ::core::clone::Clone for InkToolbarRulerButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkToolbarStencilButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkToolbarStencilButton {}
impl ::core::clone::Clone for InkToolbarStencilButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarToggleButton {}
impl ::core::clone::Clone for InkToolbarToggleButton {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for InkToolbarToolButton {}
impl ::core::clone::Clone for InkToolbarToolButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IsTextTrimmedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IsTextTrimmedChangedEventArgs {}
impl ::core::clone::Clone for IsTextTrimmedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemClickEventArgs {}
impl ::core::clone::Clone for ItemClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemClickEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemClickEventHandler {}
impl ::core::clone::Clone for ItemClickEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemCollection {}
impl ::core::clone::Clone for ItemCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemContainerGenerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemContainerGenerator {}
impl ::core::clone::Clone for ItemContainerGenerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsControl {}
impl ::core::clone::Clone for ItemsControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsPanelTemplate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsPanelTemplate {}
impl ::core::clone::Clone for ItemsPanelTemplate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsPickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsPickedEventArgs {}
impl ::core::clone::Clone for ItemsPickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsPresenter {}
impl ::core::clone::Clone for ItemsPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemsStackPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemsStackPanel {}
impl ::core::clone::Clone for ItemsStackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ItemsWrapGrid {}
impl ::core::clone::Clone for ItemsWrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ListBox {}
impl ::core::clone::Clone for ListBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListBoxItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListBoxItem {}
impl ::core::clone::Clone for ListBoxItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListPickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListPickerFlyout {}
impl ::core::clone::Clone for ListPickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListPickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListPickerFlyoutPresenter {}
impl ::core::clone::Clone for ListPickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ListView {}
impl ::core::clone::Clone for ListView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewBase {}
impl ::core::clone::Clone for ListViewBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewBaseHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewBaseHeaderItem {}
impl ::core::clone::Clone for ListViewBaseHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewHeaderItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewHeaderItem {}
impl ::core::clone::Clone for ListViewHeaderItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItem {}
impl ::core::clone::Clone for ListViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewItemToKeyHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewItemToKeyHandler {}
impl ::core::clone::Clone for ListViewItemToKeyHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewKeyToItemHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewKeyToItemHandler {}
impl ::core::clone::Clone for ListViewKeyToItemHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ListViewPersistenceHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ListViewPersistenceHelper {}
impl ::core::clone::Clone for ListViewPersistenceHelper {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MediaElement {}
impl ::core::clone::Clone for MediaElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerElement {}
impl ::core::clone::Clone for MediaPlayerElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPlayerPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPlayerPresenter {}
impl ::core::clone::Clone for MediaPlayerPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTransportControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTransportControls {}
impl ::core::clone::Clone for MediaTransportControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaTransportControlsHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaTransportControlsHelper {}
impl ::core::clone::Clone for MediaTransportControlsHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuBar {}
impl ::core::clone::Clone for MenuBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuBarItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuBarItem {}
impl ::core::clone::Clone for MenuBarItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuBarItemFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuBarItemFlyout {}
impl ::core::clone::Clone for MenuBarItemFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyout {}
impl ::core::clone::Clone for MenuFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutItem {}
impl ::core::clone::Clone for MenuFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutItemBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutItemBase {}
impl ::core::clone::Clone for MenuFlyoutItemBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutPresenter {}
impl ::core::clone::Clone for MenuFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutSeparator {}
impl ::core::clone::Clone for MenuFlyoutSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuFlyoutSubItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuFlyoutSubItem {}
impl ::core::clone::Clone for MenuFlyoutSubItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationView {}
impl ::core::clone::Clone for NavigationView {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NavigationViewBackRequestedEventArgs {}
impl ::core::clone::Clone for NavigationViewBackRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NavigationViewDisplayModeChangedEventArgs {}
impl ::core::clone::Clone for NavigationViewDisplayModeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItem {}
impl ::core::clone::Clone for NavigationViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemBase {}
impl ::core::clone::Clone for NavigationViewItemBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemHeader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemHeader {}
impl ::core::clone::Clone for NavigationViewItemHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemInvokedEventArgs {}
impl ::core::clone::Clone for NavigationViewItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewItemSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewItemSeparator {}
impl ::core::clone::Clone for NavigationViewItemSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NavigationViewList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NavigationViewList {}
impl ::core::clone::Clone for NavigationViewList {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NavigationViewPaneClosingEventArgs {}
impl ::core::clone::Clone for NavigationViewPaneClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NavigationViewSelectionChangedEventArgs {}
impl ::core::clone::Clone for NavigationViewSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for NavigationViewTemplateSettings {}
impl ::core::clone::Clone for NavigationViewTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotifyEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotifyEventArgs {}
impl ::core::clone::Clone for NotifyEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotifyEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotifyEventHandler {}
impl ::core::clone::Clone for NotifyEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for Page {}
impl ::core::clone::Clone for Page {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Panel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Panel {}
impl ::core::clone::Clone for Panel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ParallaxView {}
impl ::core::clone::Clone for ParallaxView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PasswordBox {}
impl ::core::clone::Clone for PasswordBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PasswordBoxPasswordChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PasswordBoxPasswordChangingEventArgs {}
impl ::core::clone::Clone for PasswordBoxPasswordChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PathIcon {}
impl ::core::clone::Clone for PathIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PathIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PathIconSource {}
impl ::core::clone::Clone for PathIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PersonPicture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PersonPicture {}
impl ::core::clone::Clone for PersonPicture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerConfirmedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerConfirmedEventArgs {}
impl ::core::clone::Clone for PickerConfirmedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerFlyout {}
impl ::core::clone::Clone for PickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PickerFlyoutPresenter {}
impl ::core::clone::Clone for PickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Pivot(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Pivot {}
impl ::core::clone::Clone for Pivot {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PivotItem {}
impl ::core::clone::Clone for PivotItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PivotItemEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PivotItemEventArgs {}
impl ::core::clone::Clone for PivotItemEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ProgressBar {}
impl ::core::clone::Clone for ProgressBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProgressRing(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProgressRing {}
impl ::core::clone::Clone for ProgressRing {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadioButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadioButton {}
impl ::core::clone::Clone for RadioButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatingControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatingControl {}
impl ::core::clone::Clone for RatingControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatingItemFontInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatingItemFontInfo {}
impl ::core::clone::Clone for RatingItemFontInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatingItemImageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatingItemImageInfo {}
impl ::core::clone::Clone for RatingItemImageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RatingItemInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RatingItemInfo {}
impl ::core::clone::Clone for RatingItemInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RefreshContainer {}
impl ::core::clone::Clone for RefreshContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshInteractionRatioChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RefreshInteractionRatioChangedEventArgs {}
impl ::core::clone::Clone for RefreshInteractionRatioChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RefreshRequestedEventArgs {}
impl ::core::clone::Clone for RefreshRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RefreshStateChangedEventArgs {}
impl ::core::clone::Clone for RefreshStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RefreshVisualizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RefreshVisualizer {}
impl ::core::clone::Clone for RefreshVisualizer {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RelativePanel {}
impl ::core::clone::Clone for RelativePanel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RichEditBox {}
impl ::core::clone::Clone for RichEditBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichEditBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichEditBoxSelectionChangingEventArgs {}
impl ::core::clone::Clone for RichEditBoxSelectionChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichEditBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichEditBoxTextChangingEventArgs {}
impl ::core::clone::Clone for RichEditBoxTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for RichTextBlock {}
impl ::core::clone::Clone for RichTextBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RichTextBlockOverflow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RichTextBlockOverflow {}
impl ::core::clone::Clone for RichTextBlockOverflow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RowDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RowDefinition {}
impl ::core::clone::Clone for RowDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RowDefinitionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RowDefinitionCollection {}
impl ::core::clone::Clone for RowDefinitionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ScrollContentPresenter {}
impl ::core::clone::Clone for ScrollContentPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ScrollViewer {}
impl ::core::clone::Clone for ScrollViewer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollViewerView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollViewerView {}
impl ::core::clone::Clone for ScrollViewerView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollViewerViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollViewerViewChangedEventArgs {}
impl ::core::clone::Clone for ScrollViewerViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollViewerViewChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScrollViewerViewChangingEventArgs {}
impl ::core::clone::Clone for ScrollViewerViewChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBox {}
impl ::core::clone::Clone for SearchBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBoxQueryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBoxQueryChangedEventArgs {}
impl ::core::clone::Clone for SearchBoxQueryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBoxQuerySubmittedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBoxQuerySubmittedEventArgs {}
impl ::core::clone::Clone for SearchBoxQuerySubmittedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBoxResultSuggestionChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBoxResultSuggestionChosenEventArgs {}
impl ::core::clone::Clone for SearchBoxResultSuggestionChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SearchBoxSuggestionsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SearchBoxSuggestionsRequestedEventArgs {}
impl ::core::clone::Clone for SearchBoxSuggestionsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SectionsInViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SectionsInViewChangedEventArgs {}
impl ::core::clone::Clone for SectionsInViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SectionsInViewChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SectionsInViewChangedEventHandler {}
impl ::core::clone::Clone for SectionsInViewChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SelectionChangedEventArgs {}
impl ::core::clone::Clone for SelectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectionChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SelectionChangedEventHandler {}
impl ::core::clone::Clone for SelectionChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SemanticZoom {}
impl ::core::clone::Clone for SemanticZoom {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SemanticZoomLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SemanticZoomLocation {}
impl ::core::clone::Clone for SemanticZoomLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SemanticZoomViewChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SemanticZoomViewChangedEventArgs {}
impl ::core::clone::Clone for SemanticZoomViewChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SemanticZoomViewChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SemanticZoomViewChangedEventHandler {}
impl ::core::clone::Clone for SemanticZoomViewChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SettingsFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SettingsFlyout {}
impl ::core::clone::Clone for SettingsFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Slider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Slider {}
impl ::core::clone::Clone for Slider {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SplitButton {}
impl ::core::clone::Clone for SplitButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitButtonAutomationPeer {}
impl ::core::clone::Clone for SplitButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitButtonClickEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitButtonClickEventArgs {}
impl ::core::clone::Clone for SplitButtonClickEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SplitView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SplitView {}
impl ::core::clone::Clone for SplitView {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SplitViewPaneClosingEventArgs {}
impl ::core::clone::Clone for SplitViewPaneClosingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for StackPanel {}
impl ::core::clone::Clone for StackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for StyleSelector {}
impl ::core::clone::Clone for StyleSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwapChainBackgroundPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwapChainBackgroundPanel {}
impl ::core::clone::Clone for SwapChainBackgroundPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwapChainPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwapChainPanel {}
impl ::core::clone::Clone for SwapChainPanel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SwipeControl {}
impl ::core::clone::Clone for SwipeControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwipeItem {}
impl ::core::clone::Clone for SwipeItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwipeItemInvokedEventArgs {}
impl ::core::clone::Clone for SwipeItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SwipeItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SwipeItems {}
impl ::core::clone::Clone for SwipeItems {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SymbolIcon {}
impl ::core::clone::Clone for SymbolIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SymbolIconSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SymbolIconSource {}
impl ::core::clone::Clone for SymbolIconSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBlock {}
impl ::core::clone::Clone for TextBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBox {}
impl ::core::clone::Clone for TextBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBoxBeforeTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBoxBeforeTextChangingEventArgs {}
impl ::core::clone::Clone for TextBoxBeforeTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBoxSelectionChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBoxSelectionChangingEventArgs {}
impl ::core::clone::Clone for TextBoxSelectionChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextBoxTextChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextBoxTextChangingEventArgs {}
impl ::core::clone::Clone for TextBoxTextChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextChangedEventArgs {}
impl ::core::clone::Clone for TextChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextChangedEventHandler {}
impl ::core::clone::Clone for TextChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextCommandBarFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextCommandBarFlyout {}
impl ::core::clone::Clone for TextCommandBarFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextCompositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextCompositionChangedEventArgs {}
impl ::core::clone::Clone for TextCompositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextCompositionEndedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextCompositionEndedEventArgs {}
impl ::core::clone::Clone for TextCompositionEndedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextCompositionStartedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextCompositionStartedEventArgs {}
impl ::core::clone::Clone for TextCompositionStartedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextControlCopyingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextControlCopyingToClipboardEventArgs {}
impl ::core::clone::Clone for TextControlCopyingToClipboardEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextControlCuttingToClipboardEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextControlCuttingToClipboardEventArgs {}
impl ::core::clone::Clone for TextControlCuttingToClipboardEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextControlPasteEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextControlPasteEventArgs {}
impl ::core::clone::Clone for TextControlPasteEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TextControlPasteEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TextControlPasteEventHandler {}
impl ::core::clone::Clone for TextControlPasteEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickedEventArgs {}
impl ::core::clone::Clone for TimePickedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePicker {}
impl ::core::clone::Clone for TimePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerFlyout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerFlyout {}
impl ::core::clone::Clone for TimePickerFlyout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerFlyoutPresenter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerFlyoutPresenter {}
impl ::core::clone::Clone for TimePickerFlyoutPresenter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerSelectedValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerSelectedValueChangedEventArgs {}
impl ::core::clone::Clone for TimePickerSelectedValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimePickerValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimePickerValueChangedEventArgs {}
impl ::core::clone::Clone for TimePickerValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleMenuFlyoutItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleMenuFlyoutItem {}
impl ::core::clone::Clone for ToggleMenuFlyoutItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSplitButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSplitButton {}
impl ::core::clone::Clone for ToggleSplitButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSplitButtonAutomationPeer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSplitButtonAutomationPeer {}
impl ::core::clone::Clone for ToggleSplitButtonAutomationPeer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSplitButtonIsCheckedChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSplitButtonIsCheckedChangedEventArgs {}
impl ::core::clone::Clone for ToggleSplitButtonIsCheckedChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToggleSwitch(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToggleSwitch {}
impl ::core::clone::Clone for ToggleSwitch {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToolTip(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToolTip {}
impl ::core::clone::Clone for ToolTip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToolTipService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToolTipService {}
impl ::core::clone::Clone for ToolTipService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeView {}
impl ::core::clone::Clone for TreeView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewCollapsedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewCollapsedEventArgs {}
impl ::core::clone::Clone for TreeViewCollapsedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewDragItemsCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewDragItemsCompletedEventArgs {}
impl ::core::clone::Clone for TreeViewDragItemsCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewDragItemsStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewDragItemsStartingEventArgs {}
impl ::core::clone::Clone for TreeViewDragItemsStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewExpandingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewExpandingEventArgs {}
impl ::core::clone::Clone for TreeViewExpandingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewItem {}
impl ::core::clone::Clone for TreeViewItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewItemInvokedEventArgs {}
impl ::core::clone::Clone for TreeViewItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewItemTemplateSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewItemTemplateSettings {}
impl ::core::clone::Clone for TreeViewItemTemplateSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewList {}
impl ::core::clone::Clone for TreeViewList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TreeViewNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TreeViewNode {}
impl ::core::clone::Clone for TreeViewNode {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TwoPaneView {}
impl ::core::clone::Clone for TwoPaneView {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for UIElementCollection {}
impl ::core::clone::Clone for UIElementCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserControl {}
impl ::core::clone::Clone for UserControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VariableSizedWrapGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VariableSizedWrapGrid {}
impl ::core::clone::Clone for VariableSizedWrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Viewbox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Viewbox {}
impl ::core::clone::Clone for Viewbox {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VirtualizingPanel {}
impl ::core::clone::Clone for VirtualizingPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VirtualizingStackPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VirtualizingStackPanel {}
impl ::core::clone::Clone for VirtualizingStackPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebView {}
impl ::core::clone::Clone for WebView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewBrush(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewBrush {}
impl ::core::clone::Clone for WebViewBrush {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewContentLoadingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewContentLoadingEventArgs {}
impl ::core::clone::Clone for WebViewContentLoadingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewDOMContentLoadedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewDOMContentLoadedEventArgs {}
impl ::core::clone::Clone for WebViewDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewDeferredPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewDeferredPermissionRequest {}
impl ::core::clone::Clone for WebViewDeferredPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebViewLongRunningScriptDetectedEventArgs {}
impl ::core::clone::Clone for WebViewLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewNavigationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewNavigationCompletedEventArgs {}
impl ::core::clone::Clone for WebViewNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewNavigationFailedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewNavigationFailedEventArgs {}
impl ::core::clone::Clone for WebViewNavigationFailedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewNavigationFailedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewNavigationFailedEventHandler {}
impl ::core::clone::Clone for WebViewNavigationFailedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewNavigationStartingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewNavigationStartingEventArgs {}
impl ::core::clone::Clone for WebViewNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewNewWindowRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewNewWindowRequestedEventArgs {}
impl ::core::clone::Clone for WebViewNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewPermissionRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewPermissionRequest {}
impl ::core::clone::Clone for WebViewPermissionRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewPermissionRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewPermissionRequestedEventArgs {}
impl ::core::clone::Clone for WebViewPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for WebViewSeparateProcessLostEventArgs {}
impl ::core::clone::Clone for WebViewSeparateProcessLostEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewSettings {}
impl ::core::clone::Clone for WebViewSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewUnsupportedUriSchemeIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::clone::Clone for WebViewUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewUnviewableContentIdentifiedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewUnviewableContentIdentifiedEventArgs {}
impl ::core::clone::Clone for WebViewUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebViewWebResourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebViewWebResourceRequestedEventArgs {}
impl ::core::clone::Clone for WebViewWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WrapGrid(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WrapGrid {}
impl ::core::clone::Clone for WrapGrid {
    fn clone(&self) -> Self {
        *self
    }
}
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
