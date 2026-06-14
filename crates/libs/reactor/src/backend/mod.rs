use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use super::*;

mod winui;

pub use winui::WinUIBackend;

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
    AcceptsReturn,
    ActionButton,
    ActionButtonText,
    AlignBottomWithPanel,
    AlignHCenterWithPanel,
    AlignLeftWithPanel,
    AlignRightWithPanel,
    AlignTopWithPanel,
    AlignVCenterWithPanel,
    AttachedCanvasLeft,
    AttachedCanvasTop,
    AttachedCanvasZIndex,
    AttachedGridColumn,
    AttachedGridColumnSpan,
    AttachedGridRow,
    AttachedGridRowSpan,
    AutoSuggestBox,
    AutoSuggestItems,
    AutoSuggestPlaceholder,
    Background,
    BorderBrush,
    BorderThickness,
    CanReorderTabs,
    Caption,
    ClockIdentifier,
    CloseButton,
    CloseButtonText,
    ColorValue,
    Columns,
    ColumnSpacing,
    CommandBarFlyoutCommands,
    CompactPaneLength,
    Content,
    CornerRadius,
    DayVisible,
    DefaultLabelPosition,
    Delay,
    DisplayMode,
    DisplayName,
    Fill,
    FlyoutContent,
    FlyoutPlacement,
    FontFamily,
    FontSize,
    FontWeight,
    Foreground,
    GridColumns,
    GridRows,
    GroupName,
    Header,
    Height,
    HorizontalAlignment,
    HorizontalScrollBarVisibility,
    Icon,
    ImageSource,
    Initials,
    Interval,
    IsActive,
    IsAddTabButtonVisible,
    IsAlphaEnabled,
    IsBackButtonEnabled,
    IsBackButtonVisible,
    IsBackEnabled,
    IsCalendarOpen,
    IsChecked,
    IsClosable,
    IsColorChannelTextInputVisible,
    IsColorSliderVisible,
    IsEditable,
    IsEnabled,
    IsExpanded,
    IsGroupLabelVisible,
    IsHexInputVisible,
    IsIndeterminate,
    IsLightDismissEnabled,
    IsOn,
    IsOpen,
    IsPaneOpen,
    IsPaneToggleButtonVisible,
    IsPasswordRevealButtonEnabled,
    IsPrimaryButtonEnabled,
    IsReadOnly,
    IsSecondaryButtonEnabled,
    IsSettingsVisible,
    IsTextSelectionEnabled,
    IsTodayHighlighted,
    ItemHeader,
    ItemKey,
    Items,
    LineEndpoints,
    Margin,
    MaxColumns,
    MaxHeight,
    Maximum,
    MaxRating,
    MaxWidth,
    MenuFlyoutItems,
    MenuItems,
    Message,
    MinHeight,
    Minimum,
    MinuteIncrement,
    MinWidth,
    MonthVisible,
    NavigateUri,
    Nodes,
    OffContent,
    OnContent,
    Opacity,
    OpenPaneLength,
    Orientation,
    Padding,
    PaneDisplayMode,
    PaneTitle,
    PasswordRevealMode,
    PlaceholderText,
    PlaceholderValue,
    PreferredPlacement,
    PrimaryButtonText,
    PrimaryCommands,
    Resources,
    Rows,
    RowSpacing,
    SecondaryButtonText,
    SecondaryCommands,
    SelectedIndex,
    SelectedTag,
    SelectionMode,
    Severity,
    Spacing,
    Step,
    Stretch,
    Stroke,
    StrokeThickness,
    Style,
    StyleVariant,
    Subtitle,
    Tall,
    Text,
    TextWrapping,
    TextWrappingWrap,
    Title,
    Value,
    VerticalAlignment,
    VerticalScrollBarVisibility,
    Width,
    YearVisible,
}

/// Tagged union of every value type that can appear in a [`Backend::set_prop`]
/// call. `Unset` clears a previously-applied value.
///
/// Reactor enums that mirror WinRT enums are transported as `I32` — each
/// reactor enum is `#[repr(i32)]` with discriminants matching WinRT, so the
/// backend can construct the WinRT enum directly from the integer.
#[derive(Clone, PartialEq, Debug)]
pub enum PropValue {
    Str(String),
    F64(f64),
    U16(u16),
    Bool(bool),
    I32(i32),
    Thickness(Thickness),
    Brush(Brush),
    Unset,
    GridLengths(Vec<GridLength>),
    SurfaceImageSource(SurfaceImageSource),
    LineEndpoints(LineEndpoints),
    NavMenuItems(Vec<NavViewItem>),
    StrList(Vec<String>),
    Color {
        a: u8,
        r: u8,
        g: u8,
        b: u8,
    },
    MenuBarItems(Vec<MenuBarItemDef>),
    MenuFlyoutItems(Vec<MenuItemDef>),
    TreeViewNodes(Vec<TreeNodeDef>),
    CommandBarCommands(Vec<CommandBarCommandDef>),
    CommandBarFlyoutDef {
        primary: Vec<CommandBarCommandDef>,
        secondary: Vec<CommandBarCommandDef>,
    },
    SelectorBarItems(Vec<SelectorBarItemDef>),
    Resources(HashMap<String, String>),
}

/// Closed enum of every backend-observable input event.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Event {
    ActionButtonClick,
    AddTabButtonClick,
    BackRequested,
    CalendarDateSelected,
    Checked,
    Click,
    Closed,
    CloseRequested,
    CommandBarFlyoutClick,
    ColorChanged,
    Expanding,
    ItemClicked,
    ItemInvoked,
    PaneClosed,
    PaneToggleRequested,
    PasswordChanged,
    QuerySubmitted,
    SelectedDateChanged,
    SelectedDatesChanged,
    SelectedTimeChanged,
    SelectionChanged,
    SuggestionChosen,
    TextChanged,
    Toggled,
    ValueChanged,
}

/// Typed wrapper around a callback for a specific [`Event`] payload shape.
/// Variants are named by payload type, mirroring `PropValue`.
/// The `invoke_*` accessors panic when called on a mismatching variant.
#[derive(Clone, PartialEq, Eq)]
pub enum EventHandler {
    Unit(Callback<()>),
    Bool(Callback<bool>),
    Str(Callback<String>),
    F64(Callback<f64>),
    I32(Callback<i32>),
    Color(Callback<(u8, u8, u8, u8)>),
    DateTime(Callback<DateTime>),
    TimeSpan(Callback<TimeSpan>),
}

impl fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventHandler::Unit(_) => f.write_str("EventHandler::Unit(..)"),
            EventHandler::Bool(_) => f.write_str("EventHandler::Bool(..)"),
            EventHandler::Str(_) => f.write_str("EventHandler::Str(..)"),
            EventHandler::F64(_) => f.write_str("EventHandler::F64(..)"),
            EventHandler::I32(_) => f.write_str("EventHandler::I32(..)"),
            EventHandler::Color(_) => f.write_str("EventHandler::Color(..)"),
            EventHandler::DateTime(_) => f.write_str("EventHandler::DateTime(..)"),
            EventHandler::TimeSpan(_) => f.write_str("EventHandler::TimeSpan(..)"),
        }
    }
}

impl EventHandler {
    pub fn new(cb: Callback<()>) -> Self {
        Self::Unit(cb)
    }

    pub fn from_fn<F: Fn() + 'static>(f: F) -> Self {
        Self::Unit(Callback::new(move |()| f()))
    }

    pub fn invoke(&self) {
        match self {
            EventHandler::Unit(cb) => cb.invoke(()),
            other => {
                panic!("EventHandler::invoke() called on {other:?} — use invoke_bool/invoke_string")
            }
        }
    }

    pub fn invoke_bool(&self, v: bool) {
        match self {
            EventHandler::Bool(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_bool() called on {other:?}"),
        }
    }

    pub fn invoke_string(&self, s: String) {
        match self {
            EventHandler::Str(cb) => cb.invoke(s),
            other => panic!("EventHandler::invoke_string() called on {other:?}"),
        }
    }

    pub fn invoke_f64(&self, v: f64) {
        match self {
            EventHandler::F64(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_f64() called on {other:?}"),
        }
    }

    pub fn invoke_i32(&self, v: i32) {
        match self {
            EventHandler::I32(cb) => cb.invoke(v),
            other => panic!("EventHandler::invoke_i32() called on {other:?}"),
        }
    }

    pub fn invoke_color(&self, argb: (u8, u8, u8, u8)) {
        match self {
            EventHandler::Color(cb) => cb.invoke(argb),
            other => panic!("EventHandler::invoke_color() called on {other:?}"),
        }
    }

    pub fn invoke_datetime(&self, dt: DateTime) {
        match self {
            EventHandler::DateTime(cb) => cb.invoke(dt),
            other => panic!("EventHandler::invoke_datetime() called on {other:?}"),
        }
    }

    pub fn invoke_timespan(&self, ts: TimeSpan) {
        match self {
            EventHandler::TimeSpan(cb) => cb.invoke(ts),
            other => panic!("EventHandler::invoke_timespan() called on {other:?}"),
        }
    }
}

/// UI backend the reconciler drives. Implemented by [`RecordingBackend`]
/// for tests and by `WinUIBackend` for production. New methods must have
/// default implementations so existing backends keep compiling.
pub trait Backend {
    fn create(&mut self, kind: ControlKind) -> ControlId;

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue);

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

    fn set_templated_selection_mode(&mut self, _id: ControlId, _mode: SelectionMode) {}

    fn set_templated_can_drag_items(&mut self, _id: ControlId, _value: bool) {}

    fn set_templated_can_reorder_items(&mut self, _id: ControlId, _value: bool) {}

    fn set_templated_allow_drop(&mut self, _id: ControlId, _value: bool) {}

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

    fn set_rich_text_paragraphs(&mut self, _id: ControlId, _paragraphs: &[RichTextParagraph]) {}

    fn attach_templated_realization(
        &mut self,
        _id: ControlId,
        _realize: Rc<dyn Fn(usize)>,
        _recycle: Rc<dyn Fn(usize)>,
    ) {
    }

    fn set_accessibility(&mut self, _id: ControlId, _accessibility: &AccessibilityModifiers) {}

    fn set_keyboard_accelerators(&mut self, _id: ControlId, _accelerators: &[KeyboardAccelerator]) {
    }

    /// Apply (or clear) the WinUI `ToolTipService` attached-property
    /// tooltip for `id`. Passing `None` clears any prior tooltip.
    fn set_tooltip(&mut self, _id: ControlId, _tooltip: Option<&Tooltip>) {}

    /// Attach (or clear) per-element pointer / tap callbacks for `id`.
    /// `None` removes any prior handlers; `Some(_)` replaces them.
    fn set_pointer_handlers(&mut self, _id: ControlId, _handlers: Option<&PointerHandlers>) {}

    /// Retrieve the underlying platform element as an `IInspectable` for interop.
    /// Returns `None` on non-WinUI backends or if `id` is invalid.
    fn get_native_element(&self, _id: ControlId) -> Option<windows_core::IInspectable> {
        None
    }
}

/// Recorded backend operation, used by [`RecordingBackend`] for tests.
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
        mode: SelectionMode,
    },
    SetTemplatedCanDragItems {
        id: ControlId,
        value: bool,
    },
    SetTemplatedCanReorderItems {
        id: ControlId,
        value: bool,
    },
    SetTemplatedAllowDrop {
        id: ControlId,
        value: bool,
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
    /// A stand-in [`IInspectable`] fabricated for every control so that
    /// [`get_native_element`](Backend::get_native_element) returns `Some`,
    /// mirroring the real WinUI backend (which returns the live XAML element).
    /// This lets `on_mounted` / `on_unmounted` callbacks fire in tests. The
    /// entry is removed in [`destroy`](Backend::destroy), so a destroyed
    /// control reports `None` again.
    native_elements: rustc_hash::FxHashMap<ControlId, windows_core::IInspectable>,
}

/// Builds a stand-in native [`IInspectable`] for [`RecordingBackend`]. Any
/// concrete `IInspectable` works — the reconciler only forwards it opaquely to
/// mount / unmount callbacks — so a boxed `IReference<i32>` is the cheapest
/// dependency-free choice (the real WinUI backend returns the XAML element).
fn stub_native_element() -> windows_core::IInspectable {
    windows_reference::IReference::<i32>::from(0).into()
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

    pub fn fire_datetime(&self, id: ControlId, event: Event, dt: DateTime) {
        let h = self
            .handlers
            .get(&(id, event))
            .unwrap_or_else(|| panic!("no handler for ({id}, {event:?})"));
        h.invoke_datetime(dt);
    }

    pub fn fire_timespan(&self, id: ControlId, event: Event, ts: TimeSpan) {
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
        self.native_elements.insert(id, stub_native_element());
        id
    }

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue) {
        self.ops.push(Op::SetProp {
            id,
            prop,
            value: value.clone(),
        });
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
        self.native_elements.remove(&id);

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

    fn set_templated_selection_mode(&mut self, id: ControlId, mode: SelectionMode) {
        self.ops.push(Op::SetTemplatedSelectionMode { id, mode });
    }

    fn set_templated_can_drag_items(&mut self, id: ControlId, value: bool) {
        self.ops.push(Op::SetTemplatedCanDragItems { id, value });
    }

    fn set_templated_can_reorder_items(&mut self, id: ControlId, value: bool) {
        self.ops.push(Op::SetTemplatedCanReorderItems { id, value });
    }

    fn set_templated_allow_drop(&mut self, id: ControlId, value: bool) {
        self.ops.push(Op::SetTemplatedAllowDrop { id, value });
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

    fn set_rich_text_paragraphs(&mut self, id: ControlId, paragraphs: &[RichTextParagraph]) {
        self.ops.push(Op::SetRichTextParagraphs {
            id,
            paragraphs: paragraphs.to_vec(),
        });
    }

    fn set_accessibility(&mut self, id: ControlId, accessibility: &AccessibilityModifiers) {
        self.ops.push(Op::SetAccessibility {
            id,
            accessibility: accessibility.clone(),
        });
    }

    fn set_keyboard_accelerators(&mut self, id: ControlId, accelerators: &[KeyboardAccelerator]) {
        self.ops.push(Op::SetKeyboardAccelerators {
            id,
            accelerators: accelerators.to_vec(),
        });
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&Tooltip>) {
        self.ops.push(Op::SetTooltip {
            id,
            tooltip: tooltip.cloned(),
        });
    }

    fn get_native_element(&self, id: ControlId) -> Option<windows_core::IInspectable> {
        self.native_elements.get(&id).cloned()
    }
}

// ─── Dispatcher ──────────────────────────────────────────────────────────

pub trait Dispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool;
}

/// Thread-safe variant of [`Dispatcher`] accepting `Send` closures.
/// Implementations marshal the closure onto the UI thread.
pub trait SendDispatcher: Send + Sync + 'static {
    fn enqueue_send(
        &self,
        priority: DispatcherQueuePriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool;
}

thread_local! {
    // UI thread's rerender hook, installed by `RenderHost::set_marshaller`.
    // Single-host-per-thread; replace with a per-host registry if
    // multi-host-per-thread is added.
    static UI_RERENDER: RefCell<Option<Rc<dyn Fn()>>> = const { RefCell::new(None) };
}

/// Install (or clear) the UI thread's rerender hook.
pub fn set_ui_rerender(rerender: Option<Rc<dyn Fn()>>) {
    UI_RERENDER.with(|r| {
        *r.borrow_mut() = rerender;
    });
}

/// Request a rerender on the UI thread the marshaller targets.
pub fn request_ui_rerender_on_ui_thread() {
    UI_RERENDER.with(|r| {
        if let Some(rr) = r.borrow().as_ref() {
            rr();
        }
    });
}

/// RAII guard around `set_ui_rerender`; clears the thread-local on drop.
#[must_use = "the guard restores UI_RERENDER on drop; binding it to `_` drops it immediately"]
pub struct UiRerenderGuard {
    _not_send: std::marker::PhantomData<*const ()>,
}

impl UiRerenderGuard {
    pub fn install(rerender: Rc<dyn Fn()>) -> Self {
        set_ui_rerender(Some(rerender));
        Self {
            _not_send: std::marker::PhantomData,
        }
    }
}

impl Drop for UiRerenderGuard {
    fn drop(&mut self) {
        set_ui_rerender(None);
    }
}

/// Thread-safe, clonable handle to the UI thread's render-aware
/// dispatcher. Used by `AsyncSetState` to marshal state writes back
/// onto the UI thread.
#[derive(Clone)]
pub struct UiMarshaller {
    inner: Arc<dyn SendDispatcher>,
}

impl UiMarshaller {
    pub fn new(inner: Arc<dyn SendDispatcher>) -> Self {
        Self { inner }
    }

    /// Schedule `f` to run on the UI thread at normal priority.
    pub fn dispatch<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.inner
            .enqueue_send(DispatcherQueuePriority::Normal, Box::new(f))
    }

    /// Schedule `f` to run on the UI thread at low priority.
    pub fn dispatch_low<F>(&self, f: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        self.inner
            .enqueue_send(DispatcherQueuePriority::Low, Box::new(f))
    }
}

impl fmt::Debug for UiMarshaller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiMarshaller").finish_non_exhaustive()
    }
}

/// In-memory [`SendDispatcher`] used by tests. Closures are queued in a
/// mutex-guarded `VecDeque`; call [`Self::drain`] to run them.
#[derive(Default)]
pub struct ChannelDispatcher {
    inner: Arc<ChannelDispatcherInner>,
}

#[derive(Default)]
struct ChannelDispatcherInner {
    normal: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
    low: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
}

impl ChannelDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self) -> usize {
        self.inner.normal.lock().unwrap().len() + self.inner.low.lock().unwrap().len()
    }

    pub fn drain(&self) -> usize {
        let mut fired = 0;
        loop {
            let item = {
                let mut n = self.inner.normal.lock().unwrap();
                if let Some(f) = n.pop_front() {
                    Some(f)
                } else {
                    drop(n);
                    self.inner.low.lock().unwrap().pop_front()
                }
            };
            match item {
                Some(f) => {
                    f();
                    fired += 1;
                }
                None => break,
            }
        }
        fired
    }

    pub fn handle(&self) -> Arc<dyn SendDispatcher> {
        Arc::clone(&self.inner) as Arc<dyn SendDispatcher>
    }

    pub fn marshaller(&self) -> UiMarshaller {
        UiMarshaller::new(self.handle())
    }
}

impl SendDispatcher for ChannelDispatcherInner {
    fn enqueue_send(
        &self,
        priority: DispatcherQueuePriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool {
        match priority {
            DispatcherQueuePriority::Low => self.low.lock().unwrap().push_back(f),
            _ => self.normal.lock().unwrap().push_back(f),
        }
        true
    }
}

/// In-process [`Dispatcher`] that buffers work until [`drain`](Self::drain)
/// is called. Used by tests and by `Application::run_once`.
#[derive(Default)]
pub struct RunOnDemandDispatcher {
    inner: Rc<RunOnDemandQueue>,
}

#[derive(Default)]
struct RunOnDemandQueue {
    normal: RefCell<VecDeque<Box<dyn FnOnce()>>>,
    low: RefCell<VecDeque<Box<dyn FnOnce()>>>,
}

impl RunOnDemandDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pending(&self) -> usize {
        self.inner.normal.borrow().len() + self.inner.low.borrow().len()
    }

    pub fn drain(&self) -> usize {
        let mut fired = 0;
        loop {
            let item = {
                let mut n = self.inner.normal.borrow_mut();
                if let Some(f) = n.pop_front() {
                    Some(f)
                } else {
                    drop(n);
                    self.inner.low.borrow_mut().pop_front()
                }
            };
            match item {
                Some(f) => {
                    f();
                    fired += 1;
                }
                None => break,
            }
        }
        fired
    }
}

impl Dispatcher for RunOnDemandDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        match priority {
            DispatcherQueuePriority::Low => self.inner.low.borrow_mut().push_back(f),
            _ => self.inner.normal.borrow_mut().push_back(f),
        }
        true
    }
}

// -- WinUI Dispatcher -----------------------------------------------------------

use bindings::{DispatcherQueue, DispatcherQueueHandler};

/// [`Dispatcher`] backed by the WinUI thread's `DispatcherQueue`.
pub struct WinUIDispatcher {
    queue: DispatcherQueue,
    /// Agile handle used to satisfy [`SendDispatcher`] from any thread.
    send_handle: Arc<AgileDispatcherQueue>,
}

impl WinUIDispatcher {
    pub fn for_current_thread() -> Result<Self> {
        let queue = DispatcherQueue::GetForCurrentThread()?;
        let send_handle = Arc::new(AgileDispatcherQueue {
            queue: queue.clone(),
        });
        Ok(Self { queue, send_handle })
    }

    pub fn queue(&self) -> &DispatcherQueue {
        &self.queue
    }

    /// Thread-safe handle to this dispatcher.
    pub fn send_handle(&self) -> Arc<dyn SendDispatcher> {
        Arc::clone(&self.send_handle) as Arc<dyn SendDispatcher>
    }

    /// Build a [`UiMarshaller`] backed by this dispatcher.
    pub fn marshaller(&self) -> UiMarshaller {
        UiMarshaller::new(self.send_handle())
    }
}

impl Dispatcher for WinUIDispatcher {
    fn enqueue(&self, priority: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        let slot = RefCell::new(Some(f));
        let handler = DispatcherQueueHandler::new(move || {
            if let Some(f) = slot.borrow_mut().take() {
                f();
            }
        });
        self.queue
            .TryEnqueueWithPriority(priority, &handler)
            .unwrap_or(false)
    }
}

/// Wrapper around an agile `DispatcherQueue` so closures can be posted
/// across threads. `DispatcherQueue` implements `IAgileObject`; its
/// `TryEnqueue` is callable from any thread.
struct AgileDispatcherQueue {
    queue: DispatcherQueue,
}

// SAFETY: DispatcherQueue is an agile COM object and TryEnqueue is
// documented as callable from any thread.
unsafe impl Send for AgileDispatcherQueue {}
unsafe impl Sync for AgileDispatcherQueue {}

impl SendDispatcher for AgileDispatcherQueue {
    fn enqueue_send(
        &self,
        priority: DispatcherQueuePriority,
        f: Box<dyn FnOnce() + Send + 'static>,
    ) -> bool {
        let slot = Mutex::new(Some(f));
        let handler = DispatcherQueueHandler::new(move || {
            if let Some(f) = slot.lock().unwrap().take() {
                f();
            }
        });
        self.queue
            .TryEnqueueWithPriority(priority, &handler)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn job(log: Rc<RefCell<Vec<&'static str>>>, tag: &'static str) -> Box<dyn FnOnce()> {
        Box::new(move || log.borrow_mut().push(tag))
    }

    // FIFO/priority tests for `RunOnDemandDispatcher` live in
    // `crates/tests/reactor/tests/dispatcher.rs`; this one stays here
    // because it pokes the private `inner` field directly.

    #[test]
    fn run_on_demand_dispatcher_promotes_normal_followup_over_remaining_low() {
        let dispatcher = RunOnDemandDispatcher::new();
        let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));

        let log_for_lo1 = Rc::clone(&log);
        let dispatcher_clone = dispatcher.inner.clone();
        let lo1: Box<dyn FnOnce()> = Box::new(move || {
            log_for_lo1.borrow_mut().push("lo1");

            let log2 = Rc::clone(&log_for_lo1);
            dispatcher_clone
                .normal
                .borrow_mut()
                .push_back(Box::new(move || log2.borrow_mut().push("n_followup")));
        });

        dispatcher.enqueue(DispatcherQueuePriority::Low, lo1);
        dispatcher.enqueue(DispatcherQueuePriority::Low, job(Rc::clone(&log), "lo2"));

        dispatcher.drain();
        assert_eq!(*log.borrow(), vec!["lo1", "n_followup", "lo2"]);
    }
}
