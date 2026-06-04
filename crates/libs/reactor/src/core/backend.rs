use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU32;
use std::rc::Rc;

use super::*;

/// Opaque, non-zero handle the backend assigns to every live control.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ControlId(pub NonZeroU32);

impl fmt::Display for ControlId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0.get())
    }
}

impl ControlId {
    pub fn new(raw: u32) -> Self {
        Self(NonZeroU32::new(raw).expect("ControlId must be non-zero"))
    }

    pub fn get(self) -> u32 {
        self.0.get()
    }
}

/// Closed enumeration of every control kind a backend must be able to
/// create; one variant per built-in [`Element`]
/// widget variant.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ControlKind {
    TextBlock,
    Button,
    StackPanel,
    Border,
    CheckBox,
    TextBox,
    Grid,
    ScrollViewer,
    ToggleSwitch,
    Slider,
    RadioButton,
    NumberBox,
    ProgressBar,
    ProgressRing,
    Expander,
    HyperlinkButton,
    InfoBar,
    InfoBadge,
    PersonPicture,
    Image,
    TabView,
    TabViewItem,
    NavigationView,
    TitleBar,
    Pivot,
    PivotItem,
    BreadcrumbBar,
    PasswordBox,
    RadioButtons,
    ComboBox,
    Canvas,
    Rectangle,
    Ellipse,
    Line,
    RichTextBlock,
    ListView,
    GridView,
    FlipView,
    ContentDialog,
    Viewbox,
    RepeatButton,
    RatingControl,
    ColorPicker,
    DatePicker,
    TimePicker,
    CalendarDatePicker,
    CalendarView,
    ListBox,
    DropDownButton,
    SplitButton,
    AutoSuggestBox,
    SplitView,
    MenuBar,
    ScrollView,
    TreeView,
    CommandBar,
    TeachingTip,
    SelectorBar,
    RichEditBox,
    RelativePanel,
    ToggleButton,
    SwapChainPanel,
}

/// Closed enum of every property that can be set on a control. Each
/// variant pairs with one or more [`PropValue`] kinds at runtime.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Prop {
    Text,
    FontSize,
    FontWeight,
    FontFamily,
    ButtonContent,
    IsEnabled,
    Orientation,
    Spacing,
    Margin,
    Padding,
    Width,
    Height,
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    HorizontalAlignment,
    VerticalAlignment,
    Opacity,
    Background,
    Foreground,
    IsChecked,
    CheckBoxLabel,
    TextBoxValue,
    Placeholder,
    Header,
    GridRows,
    GridColumns,
    GridRowSpacing,
    GridColumnSpacing,
    AttachedGridRow,
    AttachedGridColumn,
    AttachedGridRowSpan,
    AttachedGridColumnSpan,
    HorizontalScrollBarVisibility,
    VerticalScrollBarVisibility,
    IsOn,
    OnContent,
    OffContent,
    NumericValue,
    Minimum,
    Maximum,
    Step,
    GroupName,
    RadioLabel,
    IsIndeterminate,
    IsActive,
    IsExpanded,
    InfoBarTitle,
    InfoBarMessage,
    InfoBarSeverity,
    InfoBarIsOpen,
    /// Used by `InfoBar` and `TabViewItem` to control whether the user can
    /// dismiss/close the control.
    IsClosable,
    InfoBadgeValue,
    PersonDisplayName,
    PersonInitials,
    Fill,
    Stroke,
    StrokeThickness,
    CornerRadius,
    BorderBrush,
    BorderThickness,
    LineEndpoints,
    ImageSource,
    ImageStretch,
    SelectedIndex,
    TabHeader,
    TabItemKey,
    TabContent,
    /// Enables drag-to-reorder on `TabView` tabs.
    CanReorderTabs,
    /// Controls the visibility of the TabView's built-in add-tab button.
    IsAddTabButtonVisible,
    NavigateUri,
    NavMenuItems,
    NavSelectedTag,
    /// Whether the NavigationView has an AutoSuggestBox for search.
    NavAutoSuggestBox,
    /// Placeholder text for NavigationView's AutoSuggestBox.
    NavAutoSuggestPlaceholder,
    /// Items shown in NavigationView's AutoSuggestBox suggestion list.
    NavAutoSuggestItems,
    IsPaneOpen,
    PaneDisplayMode,
    IsBackEnabled,
    IsSettingsVisible,
    PaneTitle,
    NavHeaderString,
    TitleBarTitle,
    TitleBarSubtitle,
    TitleBarTall,
    IsBackButtonVisible,
    IsPaneToggleButtonVisible,
    PivotTitle,
    PivotItemHeader,
    BreadcrumbItems,
    IsTextSelectionEnabled,
    TextWrappingWrap,
    AcceptsReturn,
    PasswordValue,
    PasswordRevealMode,
    IsPasswordRevealButtonEnabled,
    RadioButtonsItems,
    RadioButtonsMaxColumns,
    ComboBoxItems,
    /// Non-templated items list for `ListBox`.
    ListBoxItems,
    /// Enables the editable/text-entry mode on `ComboBox`.
    IsEditable,
    AttachedCanvasLeft,
    AttachedCanvasTop,
    AttachedCanvasZIndex,
    ContentDialogTitle,
    ContentDialogBody,
    ContentDialogPrimaryText,
    ContentDialogSecondaryText,
    ContentDialogCloseText,
    ContentDialogPrimaryEnabled,
    ContentDialogSecondaryEnabled,
    ContentDialogIsOpen,
    /// Button style variant (Accent, Default).
    ButtonStyleVariant,
    /// Repeat delay in ms for `RepeatButton`.
    RepeatDelay,
    /// Repeat interval in ms for `RepeatButton`.
    RepeatInterval,
    /// Maximum rating for `RatingControl`.
    MaxRating,
    /// Caption text for `RatingControl`.
    RatingCaption,
    /// Whether `RatingControl` is read-only.
    IsReadOnly,
    /// Placeholder value for `RatingControl` (shown when Value is unset).
    PlaceholderValue,
    /// ARGB color value for `ColorPicker`.
    ColorValue,
    /// Whether alpha channel editing is enabled in `ColorPicker`.
    IsAlphaEnabled,
    /// Whether the hex input field is visible in `ColorPicker`.
    IsHexInputVisible,
    /// Whether the color slider is visible in `ColorPicker`.
    IsColorSliderVisible,
    /// Whether channel text inputs are visible in `ColorPicker`.
    IsColorChannelTextInputVisible,
    /// Whether day column is visible in `DatePicker`.
    DayVisible,
    /// Whether month column is visible in `DatePicker`.
    MonthVisible,
    /// Whether year column is visible in `DatePicker`.
    YearVisible,
    /// Clock identifier for `TimePicker` (e.g. "12HourClock", "24HourClock").
    ClockIdentifier,
    /// Minute increment for `TimePicker`.
    MinuteIncrement,
    /// Whether today is highlighted in `CalendarDatePicker`.
    IsTodayHighlighted,
    /// Whether the calendar popup is open in `CalendarDatePicker`.
    IsCalendarOpen,
    /// CalendarView: whether group labels are visible.
    IsGroupLabelVisible,
    /// Items list for `AutoSuggestBox` suggestions.
    AutoSuggestItems,
    /// Text value for `AutoSuggestBox`.
    AutoSuggestText,
    /// Display mode for `SplitView`.
    SplitViewDisplayMode,
    /// Whether the pane is open in `SplitView`.
    SplitViewIsPaneOpen,
    /// Width of the pane when fully open in `SplitView`.
    SplitViewOpenPaneLength,
    /// Width of the pane when compact in `SplitView`.
    SplitViewCompactPaneLength,
    /// Menu items for `MenuBar`.
    MenuBarItems,
    /// Menu flyout items for a button's flyout.
    MenuFlyoutItems,
    /// Tree nodes for `TreeView`.
    TreeViewNodes,
    /// Selection mode for `TreeView`.
    TreeViewSelectionMode,
    /// Primary commands for `CommandBar`.
    CommandBarPrimaryCommands,
    /// Secondary commands for `CommandBar`.
    CommandBarSecondaryCommands,
    /// Default label position for `CommandBar`.
    CommandBarDefaultLabelPosition,
    /// Primary + secondary commands for `CommandBarFlyout` on a Button.
    CommandBarFlyoutCommands,
    /// Title for `TeachingTip`.
    TeachingTipTitle,
    /// Subtitle for `TeachingTip`.
    TeachingTipSubtitle,
    /// IsOpen for `TeachingTip`.
    TeachingTipIsOpen,
    /// IsLightDismissEnabled for `TeachingTip`.
    TeachingTipIsLightDismiss,
    /// PreferredPlacement for `TeachingTip`.
    TeachingTipPlacement,
    /// ActionButtonContent for `TeachingTip`.
    TeachingTipActionButton,
    /// CloseButtonContent for `TeachingTip`.
    TeachingTipCloseButton,
    /// Items for `SelectorBar`.
    SelectorBarItems,
    /// Icon for `Button` (symbol icon rendered alongside content).
    ButtonIcon,
    /// Text value for `RichEditBox` (plain text round-trip).
    RichEditBoxText,
    /// Whether `RichEditBox` is read-only.
    RichEditBoxIsReadOnly,
    /// Flyout content element for a `Button`.
    FlyoutContent,
    /// Flyout placement mode.
    FlyoutPlacement,
    /// RelativePanel: align child to left edge of panel.
    RelativePanelAlignLeftWithPanel,
    /// RelativePanel: align child to right edge of panel.
    RelativePanelAlignRightWithPanel,
    /// RelativePanel: align child to top edge of panel.
    RelativePanelAlignTopWithPanel,
    /// RelativePanel: align child to bottom edge of panel.
    RelativePanelAlignBottomWithPanel,
    /// RelativePanel: center child horizontally in panel.
    RelativePanelAlignHCenterWithPanel,
    /// RelativePanel: center child vertically in panel.
    RelativePanelAlignVCenterWithPanel,
    /// Key/value entries written into `FrameworkElement.Resources`.
    Resources,
}


/// Tagged union of every value type that can appear in a [`Backend::set_prop`]
/// call. `Unset` clears a previously-applied value.
#[derive(Clone, PartialEq, Debug)]
pub enum PropValue {
    Str(String),
    F64(f64),
    U16(u16),
    Bool(bool),
    Thickness(Thickness),
    HAlign(HorizontalAlignment),
    VAlign(VerticalAlignment),
    Vertical(bool),
    Brush(Brush),
    Unset,
    I32(i32),
    GridLengths(Vec<GridLength>),
    ScrollVis(ScrollBarVisibility),
    InfoBarSev(InfoBarSeverity),
    ImageStretch(ImageStretch),
    LineEndpoints(LineEndpoints),
    NavMenuItems(Vec<NavViewItem>),
    NavPaneDisplayMode(NavViewPaneDisplayMode),
    StrList(Vec<String>),
    PasswordRevealMode(PasswordRevealMode),
    ButtonStyle(ButtonStyle),
    Color {
        a: u8,
        r: u8,
        g: u8,
        b: u8,
    },
    MenuBarItems(Vec<MenuBarItemDef>),
    MenuFlyoutItems(Vec<MenuItemDef>),
    TreeViewNodes(Vec<TreeNodeDef>),
    TreeViewSelectionMode(TreeSelectionMode),
    ScrollViewScrollBarVis(ScrollViewScrollBarVisibility),
    CommandBarCommands(Vec<CommandBarCommandDef>),
    CommandBarLabelPosition(CommandBarLabelPos),
    CommandBarFlyoutDef {
        primary: Vec<CommandBarCommandDef>,
        secondary: Vec<CommandBarCommandDef>,
    },
    TeachingTipPlacement(TeachingTipPlacement),
    SelectorBarItems(Vec<SelectorBarItemDef>),
    SymbolIcon(SymbolGlyph),
    FlyoutPlacement(FlyoutPlacement),
    Resources(HashMap<String, String>),
}

/// Closed enum of every backend-observable input event.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    Click,
    CheckedChanged,
    TextChanged,
    Toggled,
    ValueChanged,
    RadioChecked,
    ExpandedChanged,
    InfoBarClosed,
    TabSelectionChanged,
    TabCloseRequested,
    /// TabView: the built-in "+" add-tab button was clicked.
    AddTabButtonClick,
    NavSelectionChanged,
    NavBackRequested,
    /// NavigationView: search box query submitted.
    NavSearchQuerySubmitted,
    /// NavigationView: search box text changed.
    NavSearchTextChanged,
    /// NavigationView: search box suggestion chosen (clicked).
    NavSearchSuggestionChosen,
    TitleBarBackRequested,
    TitleBarPaneToggle,
    PivotSelectionChanged,
    BreadcrumbItemClicked,
    PasswordChanged,
    RadioButtonsSelectionChanged,
    ComboSelectionChanged,
    ContentDialogClosed,
    RatingValueChanged,
    ColorChanged,
    DateSelected,
    TimeSelected,
    CalendarDateSelected,
    /// CalendarView: dates selection changed.
    CalendarViewSelectionChanged,
    /// ListBox: selection changed (fires index).
    ListBoxSelectionChanged,
    /// SplitButton: primary action clicked.
    SplitButtonClick,
    /// AutoSuggestBox: text changed by user input.
    AutoSuggestTextChanged,
    /// AutoSuggestBox: query submitted.
    AutoSuggestQuerySubmitted,
    /// AutoSuggestBox: suggestion chosen from the list.
    AutoSuggestSuggestionChosen,
    /// SplitView: pane closed.
    SplitViewPaneClosed,
    /// MenuBar: a menu flyout item was clicked.
    MenuBarItemClicked,
    /// MenuFlyout on a button: item was clicked.
    MenuFlyoutItemClicked,
    /// TreeView: an item was invoked (clicked/tapped).
    TreeViewItemInvoked,
    /// CommandBar: a command button was clicked (label is sent).
    CommandBarClick,
    /// CommandBarFlyout: a primary/secondary command was clicked.
    CommandBarFlyoutClick,
    /// TeachingTip: closed.
    TeachingTipClosed,
    /// TeachingTip: action button clicked.
    TeachingTipActionClick,
    /// SelectorBar: selection changed (selected item text is sent).
    SelectorBarSelectionChanged,
    /// RichEditBox: text changed.
    RichEditBoxTextChanged,
    /// Flyout: opened.
    FlyoutOpened,
    /// Flyout: closed.
    FlyoutClosed,
}

/// Typed wrapper around a callback for a specific [`Event`] payload shape.
/// The `invoke_*` accessors panic when called on a mismatching variant.
#[derive(Clone, PartialEq, Eq)]
pub enum EventHandler {
    Click(Callback<()>),
    CheckedChanged(Callback<bool>),
    TextChanged(Callback<String>),
    ValueChanged(Callback<f64>),
    IndexChanged(Callback<i32>),
    TabKey(Callback<String>),
    ColorChanged(Callback<(u8, u8, u8, u8)>),
    DateTimeChanged(Callback<windows_time::DateTime>),
    TimeChanged(Callback<windows_time::TimeSpan>),
}

impl fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventHandler::Click(_) => f.write_str("EventHandler::Click(..)"),
            EventHandler::CheckedChanged(_) => f.write_str("EventHandler::CheckedChanged(..)"),
            EventHandler::TextChanged(_) => f.write_str("EventHandler::TextChanged(..)"),
            EventHandler::ValueChanged(_) => f.write_str("EventHandler::ValueChanged(..)"),
            EventHandler::IndexChanged(_) => f.write_str("EventHandler::IndexChanged(..)"),
            EventHandler::TabKey(_) => f.write_str("EventHandler::TabKey(..)"),
            EventHandler::ColorChanged(_) => f.write_str("EventHandler::ColorChanged(..)"),
            EventHandler::DateTimeChanged(_) => f.write_str("EventHandler::DateTimeChanged(..)"),
            EventHandler::TimeChanged(_) => f.write_str("EventHandler::TimeChanged(..)"),
        }
    }
}

impl EventHandler {
    pub fn new(cb: Callback<()>) -> Self {
        Self::Click(cb)
    }

    pub fn from_fn<F: Fn() + 'static>(f: F) -> Self {
        Self::Click(Callback::new(move |()| f()))
    }

    pub fn invoke(&self) {
        match self {
            EventHandler::Click(cb) => cb.invoke(()),
            other => {
                panic!("EventHandler::invoke() called on {other:?} — use invoke_bool/invoke_string")
            }
        }
    }

    pub fn invoke_bool(&self, v: bool) {
        match self {
            EventHandler::CheckedChanged(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_bool() called on {other:?}"),
        }
    }

    pub fn invoke_string(&self, s: String) {
        match self {
            EventHandler::TextChanged(cb) | EventHandler::TabKey(cb) => cb.invoke(s),
            other => panic!("EventHandler::invoke_string() called on {other:?}"),
        }
    }

    pub fn invoke_f64(&self, v: f64) {
        match self {
            EventHandler::ValueChanged(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_f64() called on {other:?}"),
        }
    }

    pub fn invoke_i32(&self, v: i32) {
        match self {
            EventHandler::IndexChanged(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_i32() called on {other:?}"),
        }
    }

    pub fn invoke_color(&self, argb: (u8, u8, u8, u8)) {
        match self {
            EventHandler::ColorChanged(cb) => cb.invoke(argb),
            other => panic!("EventHandler::invoke_color() called on {other:?}"),
        }
    }

    pub fn invoke_datetime(&self, dt: windows_time::DateTime) {
        match self {
            EventHandler::DateTimeChanged(cb) => cb.invoke(dt),
            other => panic!("EventHandler::invoke_datetime() called on {other:?}"),
        }
    }

    pub fn invoke_timespan(&self, ts: windows_time::TimeSpan) {
        match self {
            EventHandler::TimeChanged(cb) => cb.invoke(ts),
            other => panic!("EventHandler::invoke_timespan() called on {other:?}"),
        }
    }
}

/// UI backend the reconciler drives. Implemented by [`RecordingBackend`]
/// for tests and by `WinUIBackend` for production. New methods must have
/// default implementations so existing backends keep compiling.
pub trait Backend {
    fn create(&mut self, kind: ControlKind) -> ControlId;

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: PropValue);

    /// Typed property mount: the backend receives the concrete widget struct
    /// (as `&dyn Any`) and calls COM setters directly. Returns `true` if handled.
    fn mount_props(&mut self, _id: ControlId, _widget: &dyn std::any::Any) -> bool {
        false
    }

    /// Typed property diff: receives old and new widget structs, calls setters
    /// only for changed fields. Returns `true` if handled.
    fn diff_props(
        &mut self,
        _id: ControlId,
        _old: &dyn std::any::Any,
        _new: &dyn std::any::Any,
    ) -> bool {
        false
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId);

    fn remove_child(&mut self, parent: ControlId, index: usize);

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId);

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize);

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId);

    fn destroy(&mut self, id: ControlId);

    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler);

    fn detach_event(&mut self, _id: ControlId, _event: Event) {}

    fn set_templated_item_count(&mut self, _id: ControlId, _count: usize) {}

    fn set_templated_row_content(
        &mut self,
        _list_id: ControlId,
        _row_idx: usize,
        _content: Option<ControlId>,
    ) {
    }

    fn set_templated_selected_index(&mut self, _id: ControlId, _index: i32) {}

    fn set_templated_selection_mode(
        &mut self,
        _id: ControlId,
        _mode: crate::core::templated_list::SelectionMode,
    ) {
    }

    /// Set a mounted element tree as the header content of a control (e.g.
    /// Expander). Pass `None` to clear a previously set element header.
    fn set_header_element(&mut self, _id: ControlId, _header_id: Option<ControlId>) {}

    /// Set a mounted element tree as the pane content of a `SplitView`.
    /// Pass `None` to clear a previously set pane element.
    fn set_pane_element(&mut self, _id: ControlId, _pane_id: Option<ControlId>) {}

    /// W1: scroll a templated list to the specified item index. Default
    /// no-op; the WinUI backend implements this via
    /// `ListViewBase::ScrollIntoView` when `id` resolves to a list/grid/flip
    /// view. Negative indices are ignored.
    fn scroll_templated_to_index(&mut self, _id: ControlId, _index: i32) {}

    fn attach_templated_selection_changed(&mut self, _id: ControlId, _handler: Callback<i32>) {}

    fn set_theme_bindings(
        &mut self,
        _id: ControlId,
        _kind: ControlKind,
        _bindings: &[(Prop, ThemeRef)],
    ) {
    }

    fn on_theme_changed(&mut self) {}

    fn set_implicit_transitions(
        &mut self,
        _id: ControlId,
        _transitions: Option<ImplicitTransitions>,
    ) {
    }

    fn set_layout_animation(&mut self, _id: ControlId, _config: Option<LayoutAnimationConfig>) {}

    fn run_property_animation(&mut self, _id: ControlId, _config: Option<AnimationConfig>) {}

    fn set_rich_text_paragraphs(
        &mut self,
        _id: ControlId,
        _paragraphs: &[crate::core::rich_text::RichTextParagraph],
    ) {
    }

    fn attach_templated_realization(
        &mut self,
        _id: ControlId,
        _realize: Rc<dyn Fn(usize)>,
        _recycle: Rc<dyn Fn(usize)>,
    ) {
    }

    fn set_accessibility(
        &mut self,
        _id: ControlId,
        _accessibility: &AccessibilityModifiers,
    ) {
    }

    fn set_keyboard_accelerators(
        &mut self,
        _id: ControlId,
        _accelerators: &[KeyboardAccelerator],
    ) {
    }

    /// Apply (or clear) the WinUI `ToolTipService` attached-property
    /// tooltip for `id`. Passing `None` clears any prior tooltip.
    fn set_tooltip(&mut self, _id: ControlId, _tooltip: Option<&Tooltip>) {}

    /// Attach (or clear) per-element pointer / tap callbacks for `id`.
    /// `None` removes any prior handlers; `Some(_)` replaces them.
    fn set_pointer_handlers(
        &mut self,
        _id: ControlId,
        _handlers: Option<&crate::core::pointer::PointerHandlers>,
    ) {
    }

    /// Retrieve the underlying platform element as an `IInspectable` for interop.
    /// Returns `None` on non-WinUI backends or if `id` is invalid.
    fn get_native_element(&self, _id: ControlId) -> Option<windows_core::IInspectable> {
        None
    }
}

/// Recorded backend operation, used by [`RecordingBackend`] for tests.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Op {
    Create {
        id: ControlId,
        kind: ControlKind,
    },
    SetProp {
        id: ControlId,
        prop: Prop,
        value: PropValue,
    },
    AppendChild {
        parent: ControlId,
        child: ControlId,
    },
    RemoveChild {
        parent: ControlId,
        index: usize,
    },
    ReplaceChild {
        parent: ControlId,
        index: usize,
        new: ControlId,
    },
    MoveChild {
        parent: ControlId,
        from: usize,
        to: usize,
    },
    InsertChild {
        parent: ControlId,
        index: usize,
        child: ControlId,
    },
    Destroy {
        id: ControlId,
    },
    AttachEvent {
        id: ControlId,
        event: Event,
        handler: EventHandler,
    },
    DetachEvent {
        id: ControlId,
        event: Event,
    },
    SetTemplatedItemCount {
        list_id: ControlId,
        count: usize,
    },
    MountRowContent {
        list_id: ControlId,
        row_idx: usize,
        content: ControlId,
    },
    ClearRowContent {
        list_id: ControlId,
        row_idx: usize,
    },
    SetTemplatedSelectedIndex {
        id: ControlId,
        index: i32,
    },
    SetTemplatedSelectionMode {
        id: ControlId,
        mode: crate::core::templated_list::SelectionMode,
    },
    SetHeaderElement {
        id: ControlId,
        header_id: Option<ControlId>,
    },
    SetPaneElement {
        id: ControlId,
        pane_id: Option<ControlId>,
    },
    ScrollTemplatedToIndex {
        id: ControlId,
        index: i32,
    },
    AttachTemplatedSelectionChanged {
        id: ControlId,
    },
    AttachTemplatedRealization {
        id: ControlId,
    },
    ApplyThemeBindings {
        id: ControlId,
        kind: ControlKind,
        bindings: Vec<(Prop, ThemeRef)>,
        cache_hit: bool,
    },
    OnThemeChanged,
    SetImplicitTransitions {
        id: ControlId,
        transitions: Option<ImplicitTransitions>,
    },
    SetLayoutAnimation {
        id: ControlId,
        config: Option<LayoutAnimationConfig>,
    },
    RunPropertyAnimation {
        id: ControlId,
        config: Option<AnimationConfig>,
    },
    SetRichTextParagraphs {
        id: ControlId,
        paragraphs: Vec<RichTextParagraph>,
    },
    SetAccessibility {
        id: ControlId,
        accessibility: AccessibilityModifiers,
    },
    SetKeyboardAccelerators {
        id: ControlId,
        accelerators: Vec<KeyboardAccelerator>,
    },
    SetTooltip {
        id: ControlId,
        tooltip: Option<Tooltip>,
    },
}

/// In-memory [`Backend`] that records every operation as an [`Op`] and
/// can replay events back into the recorded handlers; used by the test
/// suite as the deterministic counterpart to `WinUIBackend`.
#[doc(hidden)]
#[derive(Default)]
pub struct RecordingBackend {
    pub ops: Vec<Op>,
    next_id: u32,
    children: rustc_hash::FxHashMap<ControlId, Vec<ControlId>>,
    handlers: rustc_hash::FxHashMap<(ControlId, Event), EventHandler>,
    row_contents: rustc_hash::FxHashMap<ControlId, rustc_hash::FxHashMap<usize, ControlId>>,
    item_counts: rustc_hash::FxHashMap<ControlId, usize>,
    realization_handlers: rustc_hash::FxHashMap<ControlId, (Rc<dyn Fn(usize)>, Rc<dyn Fn(usize)>)>,
    selection_handlers: rustc_hash::FxHashMap<ControlId, Callback<i32>>,
    theme_style_cache: rustc_hash::FxHashSet<(ControlKind, Vec<(Prop, ThemeRef)>)>,
    theme_bindings_live: rustc_hash::FxHashMap<ControlId, Vec<(Prop, ThemeRef)>>,
}

impl RecordingBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn op_count(&self) -> usize {
        self.ops.len()
    }

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    pub fn children_of(&self, parent: ControlId) -> &[ControlId] {
        self.children.get(&parent).map_or(&[], Vec::as_slice)
    }

    pub fn fire(&self, id: ControlId, event: Event) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke();
    }

    pub fn fire_bool(&self, id: ControlId, event: Event, v: bool) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_bool(v);
    }

    pub fn fire_string(&self, id: ControlId, event: Event, s: String) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_string(s);
    }

    pub fn fire_f64(&self, id: ControlId, event: Event, v: f64) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_f64(v);
    }

    pub fn fire_i32(&self, id: ControlId, event: Event, v: i32) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_i32(v);
    }

    pub fn fire_datetime(&self, id: ControlId, event: Event, dt: windows_time::DateTime) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_datetime(dt);
    }

    pub fn fire_timespan(&self, id: ControlId, event: Event, ts: windows_time::TimeSpan) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_timespan(ts);
    }

    pub fn row_contents_of(&self, list_id: ControlId) -> rustc_hash::FxHashMap<usize, ControlId> {
        self.row_contents.get(&list_id).cloned().unwrap_or_default()
    }

    pub fn item_count_of(&self, list_id: ControlId) -> Option<usize> {
        self.item_counts.get(&list_id).copied()
    }

    pub fn simulate_prepare_row(&self, list_id: ControlId, row_idx: usize) {
        let (realize, _recycle) = self
            .realization_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no realization handler attached to {list_id}"));
        realize(row_idx);
    }

    pub fn simulate_clear_row(&self, list_id: ControlId, row_idx: usize) {
        let (_realize, recycle) = self
            .realization_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no realization handler attached to {list_id}"));
        recycle(row_idx);
    }

    pub fn fire_templated_selection_changed(&self, list_id: ControlId, index: i32) {
        let cb = self
            .selection_handlers
            .get(&list_id)
            .unwrap_or_else(|| panic!("no selection handler on {list_id}"));
        cb.invoke(index);
    }

    pub fn theme_cache_size(&self) -> usize {
        self.theme_style_cache.len()
    }

    pub fn theme_bindings_of(&self, id: ControlId) -> Vec<(Prop, ThemeRef)> {
        self.theme_bindings_live
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn live_control_count(&self) -> usize {
        let mut alive: rustc_hash::FxHashSet<ControlId> = rustc_hash::FxHashSet::default();
        for op in &self.ops {
            match op {
                Op::Create { id, .. } => {
                    alive.insert(*id);
                }
                Op::Destroy { id } => {
                    alive.remove(id);
                }
                _ => {}
            }
        }
        alive.len()
    }
}

impl Backend for RecordingBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        self.next_id += 1;
        let id = ControlId::new(self.next_id);
        self.ops.push(Op::Create { id, kind });
        id
    }

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: PropValue) {
        self.ops.push(Op::SetProp { id, prop, value });
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        self.children.entry(parent).or_default().push(child);
        self.ops.push(Op::AppendChild { parent, child });
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) {
        let list = self.children.entry(parent).or_default();
        assert!(
            index < list.len(),
            "remove_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list.remove(index);
        self.ops.push(Op::RemoveChild { parent, index });
    }

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        let list = self.children.entry(parent).or_default();
        assert!(
            index < list.len(),
            "replace_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list[index] = new;
        self.ops.push(Op::ReplaceChild { parent, index, new });
    }

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        let list = self.children.entry(parent).or_default();
        assert!(
            from < list.len(),
            "move_child: from-index {from} out of bounds for parent {parent} (len={})",
            list.len()
        );
        assert!(
            to < list.len(),
            "move_child: to-index {to} out of bounds for parent {parent} (len={})",
            list.len()
        );
        if from != to {
            let item = list.remove(from);
            list.insert(to, item);
        }
        self.ops.push(Op::MoveChild { parent, from, to });
    }

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let list = self.children.entry(parent).or_default();
        assert!(
            index <= list.len(),
            "insert_child: index {index} out of bounds for parent {parent} (len={})",
            list.len()
        );
        list.insert(index, child);
        self.ops.push(Op::InsertChild {
            parent,
            index,
            child,
        });
    }

    fn destroy(&mut self, id: ControlId) {
        self.children.remove(&id);

        self.handlers.retain(|(hid, _), _| *hid != id);
        self.ops.push(Op::Destroy { id });
    }

    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        self.handlers.insert((id, event), handler.clone());
        self.ops.push(Op::AttachEvent { id, event, handler });
    }

    fn detach_event(&mut self, id: ControlId, event: Event) {
        self.handlers.remove(&(id, event));
        self.ops.push(Op::DetachEvent { id, event });
    }

    fn set_templated_item_count(&mut self, id: ControlId, count: usize) {
        self.item_counts.insert(id, count);
        self.ops
            .push(Op::SetTemplatedItemCount { list_id: id, count });
    }

    fn set_templated_row_content(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
        content: Option<ControlId>,
    ) {
        let slot = self.row_contents.entry(list_id).or_default();
        if let Some(c) = content {
            slot.insert(row_idx, c);
            self.ops.push(Op::MountRowContent {
                list_id,
                row_idx,
                content: c,
            });
        } else {
            slot.remove(&row_idx);
            self.ops.push(Op::ClearRowContent { list_id, row_idx });
        }
    }

    fn set_templated_selected_index(&mut self, id: ControlId, index: i32) {
        self.ops.push(Op::SetTemplatedSelectedIndex { id, index });
    }

    fn set_templated_selection_mode(
        &mut self,
        id: ControlId,
        mode: crate::core::templated_list::SelectionMode,
    ) {
        self.ops.push(Op::SetTemplatedSelectionMode { id, mode });
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        self.ops.push(Op::SetHeaderElement { id, header_id });
    }

    fn set_pane_element(&mut self, id: ControlId, pane_id: Option<ControlId>) {
        self.ops.push(Op::SetPaneElement { id, pane_id });
    }

    fn scroll_templated_to_index(&mut self, id: ControlId, index: i32) {
        self.ops.push(Op::ScrollTemplatedToIndex { id, index });
    }

    fn attach_templated_selection_changed(&mut self, id: ControlId, handler: Callback<i32>) {
        self.selection_handlers.insert(id, handler);
        self.ops.push(Op::AttachTemplatedSelectionChanged { id });
    }

    fn attach_templated_realization(
        &mut self,
        id: ControlId,
        realize: Rc<dyn Fn(usize)>,
        recycle: Rc<dyn Fn(usize)>,
    ) {
        self.realization_handlers.insert(id, (realize, recycle));
        self.ops.push(Op::AttachTemplatedRealization { id });
    }

    fn set_theme_bindings(
        &mut self,
        id: ControlId,
        kind: ControlKind,
        bindings: &[(Prop, ThemeRef)],
    ) {
        let mut canonical: Vec<(Prop, ThemeRef)> = bindings.to_vec();
        canonical.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0)));

        let cache_hit = !self.theme_style_cache.insert((kind, canonical.clone()));
        self.theme_bindings_live.insert(id, canonical.clone());
        self.ops.push(Op::ApplyThemeBindings {
            id,
            kind,
            bindings: canonical,
            cache_hit,
        });
    }

    fn on_theme_changed(&mut self) {
        self.theme_style_cache.clear();

        self.ops.push(Op::OnThemeChanged);
    }

    fn set_implicit_transitions(
        &mut self,
        id: ControlId,
        transitions: Option<ImplicitTransitions>,
    ) {
        self.ops
            .push(Op::SetImplicitTransitions { id, transitions });
    }

    fn set_layout_animation(&mut self, id: ControlId, config: Option<LayoutAnimationConfig>) {
        self.ops.push(Op::SetLayoutAnimation { id, config });
    }

    fn run_property_animation(&mut self, id: ControlId, config: Option<AnimationConfig>) {
        self.ops.push(Op::RunPropertyAnimation { id, config });
    }

    fn set_rich_text_paragraphs(
        &mut self,
        id: ControlId,
        paragraphs: &[crate::core::rich_text::RichTextParagraph],
    ) {
        self.ops.push(Op::SetRichTextParagraphs {
            id,
            paragraphs: paragraphs.to_vec(),
        });
    }

    fn set_accessibility(
        &mut self,
        id: ControlId,
        accessibility: &AccessibilityModifiers,
    ) {
        self.ops.push(Op::SetAccessibility {
            id,
            accessibility: accessibility.clone(),
        });
    }

    fn set_keyboard_accelerators(
        &mut self,
        id: ControlId,
        accelerators: &[KeyboardAccelerator],
    ) {
        self.ops.push(Op::SetKeyboardAccelerators {
            id,
            accelerators: accelerators.to_vec(),
        });
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&crate::core::tooltip::Tooltip>) {
        self.ops.push(Op::SetTooltip {
            id,
            tooltip: tooltip.cloned(),
        });
    }
}
