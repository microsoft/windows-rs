use std::collections::BTreeMap;

use windows_time::{DateTime, TimeSpan};

use crate::element::*;
use crate::framework_properties::FrameworkProps;
use crate::references::NativeWindowRef;

#[derive(Clone)]
pub struct ApplicationProps {
    pub resources: ApplicationResources,
}

#[derive(Clone)]
pub struct WindowProps {
    pub title: String,
    pub backdrop: Option<WindowBackdrop>,
    pub icon: Option<WindowIcon>,
    pub theme: WindowTheme,
    pub title_bar: SystemTitleBar,
    pub overlapped: WindowOverlappedPolicy,
    pub client_size: Option<WindowSize>,
    pub constraints: WindowConstraints,
    pub presenter: WindowPresenter,
    pub on_close_requested: EventFn,
    pub on_size_changed: WindowSizeEventFn,
    pub on_color_scheme_changed: ColorSchemeEventFn,
    pub reference: Option<NativeWindowRef>,
}

pub struct TitleBarProps {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub back_button_visible: bool,
    pub back_button_enabled: bool,
    pub pane_toggle_button_visible: bool,
    pub height: TitleBarHeight,
    pub on_back_requested: EventFn,
    pub on_pane_requested: EventFn,
}

pub struct StackPanelProps {
    pub children: Vec<Element>,
    pub orientation: Orientation,
    pub spacing: f64,
    pub padding: Option<Thickness>,
    pub framework: FrameworkProps,
}

pub struct StackPanelState {
    pub orientation: Orientation,
    pub spacing: f64,
    pub padding: Option<Thickness>,
    pub framework: FrameworkProps,
}

pub struct PanelProps {
    pub children: Vec<Element>,
    pub framework: FrameworkProps,
}

pub struct GridProps {
    pub children: Vec<Element>,
    pub columns: Vec<GridLength>,
    pub rows: Vec<GridLength>,
    pub column_spacing: f64,
    pub row_spacing: f64,
    pub framework: FrameworkProps,
}

pub struct BorderProps {
    pub background: Option<Brush>,
    pub border_brush: Option<Brush>,
    pub border_thickness: Option<Thickness>,
    pub corner_radius: Option<CornerRadius>,
    pub padding: Option<Thickness>,
    pub framework: FrameworkProps,
}

pub struct ShapeProps {
    pub kind: ShapeKind,
    pub fill: Option<Brush>,
    pub stroke: Option<Brush>,
    pub stroke_thickness: Option<f64>,
    pub corner_radius: Option<f64>,
    pub line: [f64; 4],
    pub framework: FrameworkProps,
}

pub struct GridState {
    pub columns: Vec<GridLength>,
    pub rows: Vec<GridLength>,
    pub column_spacing: f64,
    pub row_spacing: f64,
    pub framework: FrameworkProps,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPlacement {
    pub row: i32,
    pub column: i32,
    pub row_span: i32,
    pub column_span: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachedPlacement {
    Grid(GridPlacement),
    Canvas(CanvasPlacement),
    RelativePanel(RelativePanelPlacement),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RelativePanelPlacement(pub u16);

impl RelativePanelPlacement {
    pub const LEFT: u32 = 0;
    pub const RIGHT: u32 = 2;
    pub const TOP: u32 = 4;
    pub const BOTTOM: u32 = 6;
    pub const HORIZONTAL_CENTER: u32 = 8;
    pub const VERTICAL_CENTER: u32 = 10;

    pub fn set(&mut self, shift: u32, value: Option<bool>) {
        self.0 &= !(0b11 << shift);
        self.0 |= match value {
            None => 0,
            Some(false) => 1 << shift,
            Some(true) => 2 << shift,
        };
    }

    fn get(self, shift: u32) -> Option<bool> {
        match (self.0 >> shift) & 0b11 {
            0 => None,
            1 => Some(false),
            2 => Some(true),
            _ => unreachable!(),
        }
    }

    pub fn align_left(self) -> Option<bool> {
        self.get(Self::LEFT)
    }

    pub fn align_right(self) -> Option<bool> {
        self.get(Self::RIGHT)
    }

    pub fn align_top(self) -> Option<bool> {
        self.get(Self::TOP)
    }

    pub fn align_bottom(self) -> Option<bool> {
        self.get(Self::BOTTOM)
    }

    pub fn align_horizontal_center(self) -> Option<bool> {
        self.get(Self::HORIZONTAL_CENTER)
    }

    pub fn align_vertical_center(self) -> Option<bool> {
        self.get(Self::VERTICAL_CENTER)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CanvasPlacement {
    pub left: f64,
    pub top: f64,
    pub z_index: i32,
    pub flags: u32,
}

impl CanvasPlacement {
    pub const LEFT: u32 = 1;
    pub const TOP: u32 = 2;
    pub const Z_INDEX: u32 = 4;

    pub fn left(self) -> Option<f64> {
        (self.flags & Self::LEFT != 0).then_some(self.left)
    }

    pub fn top(self) -> Option<f64> {
        (self.flags & Self::TOP != 0).then_some(self.top)
    }

    pub fn z_index(self) -> Option<i32> {
        (self.flags & Self::Z_INDEX != 0).then_some(self.z_index)
    }
}

impl AttachedPlacement {
    pub fn default_for(self) -> Self {
        match self {
            Self::Grid(_) => Self::Grid(GridPlacement::default()),
            Self::Canvas(_) => Self::Canvas(CanvasPlacement::default()),
            Self::RelativePanel(_) => Self::RelativePanel(RelativePanelPlacement::default()),
        }
    }
}

impl GridPlacement {
    pub fn row(self) -> Option<i32> {
        (self.row >= 0).then_some(self.row)
    }

    pub fn column(self) -> Option<i32> {
        (self.column >= 0).then_some(self.column)
    }

    pub fn row_span(self) -> Option<i32> {
        (self.row_span != 0).then_some(self.row_span)
    }

    pub fn column_span(self) -> Option<i32> {
        (self.column_span != 0).then_some(self.column_span)
    }
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self {
            row: -1,
            column: -1,
            row_span: 0,
            column_span: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VirtualCollectionKind {
    ListView,
    GridView,
}

pub struct VirtualCollectionProps {
    pub kind: VirtualCollectionKind,
    pub items: VirtualCollectionItems,
    pub height: f64,
    pub empty: Option<Box<Element>>,
    pub automation_name: Option<String>,
    pub help_text: Option<String>,
    pub selection_mode: SelectionMode,
    pub selection: CollectionSelection,
    pub on_selection_changed: Option<SelectionEventFn>,
    pub on_item_invoked: Option<KeyEventFn>,
    pub selection_display_only: bool,
    pub can_reorder_items: bool,
    pub on_items_reordered: Option<KeysEventFn>,
    pub row: RowFn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VirtualCollectionItems {
    Implicit(usize),
    Keyed(VirtualItemKeys),
}

impl VirtualCollectionItems {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Implicit(count) => *count == 0,
            Self::Keyed(keys) => keys.is_empty(),
        }
    }

    pub fn key(&self, index: usize) -> Option<u64> {
        match self {
            Self::Implicit(count) => (index < *count).then_some(index as u64),
            Self::Keyed(keys) => keys.as_slice().get(index).copied(),
        }
    }
}

pub struct ButtonProps {
    pub on_click: Option<EventFn>,
    pub emphasis: ButtonEmphasis,
    pub framework: FrameworkProps,
}

pub struct DropDownButtonProps {
    pub on_opened: Option<EventFn>,
    pub on_closed: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct SplitButtonProps {
    pub on_click: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct FlyoutProps {
    pub placement: FlyoutPlacement,
    pub on_opened: Option<EventFn>,
    pub on_closed: Option<EventFn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MenuItemSpec {
    Item {
        key: u64,
        text: String,
        enabled: bool,
    },
    Separator {
        key: u64,
    },
    Submenu {
        key: u64,
        text: String,
        items: Vec<Self>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuBarItemSpec {
    pub key: u64,
    pub title: String,
    pub items: Vec<MenuItemSpec>,
}

pub struct MenuFlyoutProps {
    pub items: Vec<MenuItemSpec>,
    pub handlers: BTreeMap<u64, EventFn>,
    pub placement: FlyoutPlacement,
    pub on_opened: Option<EventFn>,
    pub on_closed: Option<EventFn>,
}

pub struct MenuBarProps {
    pub items: Vec<MenuBarItemSpec>,
    pub handlers: BTreeMap<u64, EventFn>,
    pub framework: FrameworkProps,
}

pub struct CommandBarFlyoutProps {
    pub primary: Vec<CommandBarItem>,
    pub secondary: Vec<CommandBarItem>,
    pub placement: FlyoutPlacement,
    pub on_opened: Option<EventFn>,
    pub on_closed: Option<EventFn>,
}

pub struct HyperlinkButtonProps {
    pub navigate_uri: Option<Box<str>>,
    pub on_click: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct RepeatButtonProps {
    pub delay: i32,
    pub interval: i32,
    pub on_click: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct ToggleButtonProps {
    pub checked: bool,
    pub on_toggle: Option<BoolEventFn>,
    pub on_click: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct ToggleSwitchProps {
    pub on: bool,
    pub header: Option<String>,
    pub on_content: Option<String>,
    pub off_content: Option<String>,
    pub on_toggle: Option<BoolEventFn>,
    pub framework: FrameworkProps,
}

pub struct InfoBadgeProps {
    pub value: Option<i32>,
    pub framework: FrameworkProps,
}

pub struct InfoBarProps {
    pub title: String,
    pub message: String,
    pub severity: InfoBarSeverity,
    pub open: bool,
    pub closable: bool,
    pub on_close_requested: Option<EventFn>,
    pub framework: FrameworkProps,
}

pub struct PersonPictureProps {
    pub display_name: Option<String>,
    pub initials: Option<String>,
    pub framework: FrameworkProps,
}

pub struct ProgressBarProps {
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub indeterminate: bool,
    pub framework: FrameworkProps,
}

pub struct ProgressRingProps {
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub active: bool,
    pub indeterminate: bool,
    pub framework: FrameworkProps,
}

pub struct SliderProps {
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub step: f64,
    pub header: Option<String>,
    pub orientation: Orientation,
    pub on_change: Option<FloatEventFn>,
    pub framework: FrameworkProps,
}

pub struct NumberBoxProps {
    pub value: Option<f64>,
    pub minimum: f64,
    pub maximum: f64,
    pub header: Option<String>,
    pub on_change: Option<OptionalFloatEventFn>,
    pub framework: FrameworkProps,
}

pub struct RatingControlProps {
    pub value: Option<f64>,
    pub max_rating: i32,
    pub placeholder: Option<f64>,
    pub caption: String,
    pub read_only: bool,
    pub on_change: Option<OptionalFloatEventFn>,
    pub framework: FrameworkProps,
}

pub struct ColorPickerProps {
    pub color: Color,
    pub alpha_enabled: bool,
    pub hex_input_visible: bool,
    pub color_slider_visible: bool,
    pub color_channel_text_input_visible: bool,
    pub on_change: Option<ColorEventFn>,
    pub framework: FrameworkProps,
}

pub struct DatePickerProps {
    pub date: Option<DateTime>,
    pub header: Option<String>,
    pub day_visible: bool,
    pub month_visible: bool,
    pub year_visible: bool,
    pub on_change: Option<OptionalDateEventFn>,
    pub framework: FrameworkProps,
}

pub struct CalendarDatePickerProps {
    pub date: Option<DateTime>,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub today_highlighted: bool,
    pub on_change: Option<OptionalDateEventFn>,
    pub framework: FrameworkProps,
}

pub struct TimePickerProps {
    pub time: Option<TimeSpan>,
    pub header: Option<String>,
    pub minute_increment: i32,
    pub on_change: Option<OptionalTimeEventFn>,
    pub framework: FrameworkProps,
}

pub struct CalendarViewProps {
    pub selected_dates: Rc<[DateTime]>,
    pub selection_mode: CalendarSelectionMode,
    pub today_highlighted: bool,
    pub group_label_visible: bool,
    pub on_change: Option<DatesEventFn>,
    pub framework: FrameworkProps,
}

pub struct NavigationViewProps {
    pub selected_key: Option<u64>,
    pub header: Option<String>,
    pub pane_title: Option<String>,
    pub settings_visible: bool,
    pub pane_toggle_visible: bool,
    pub pane_open: bool,
    pub open_pane_length: f64,
    pub pane_display_mode: NavigationPaneDisplayMode,
    pub on_selection_changed: Option<OptionalKeyEventFn>,
    pub on_pane_open_changed: Option<BoolEventFn>,
    pub on_display_mode_changed: Option<NavigationDisplayModeEventFn>,
    pub framework: FrameworkProps,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationViewItemProps {
    pub item_key: u64,
    pub label: String,
    pub icon: Option<Icon>,
}

pub struct RichEditBoxProps {
    pub text: String,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub read_only: bool,
    pub on_change: Option<TextEventFn>,
    pub framework: FrameworkProps,
}

pub struct RichTextBlockProps {
    pub paragraphs: Rc<[RichTextParagraph]>,
    pub font_size: Option<f64>,
    pub selectable: bool,
    pub wrap: bool,
    pub framework: FrameworkProps,
}

pub struct TreeViewProps {
    pub nodes: Rc<[TreeNode]>,
    pub on_expanded_changed: Option<KeyBoolEventFn>,
    pub on_item_invoked: Option<KeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct ListBoxProps {
    pub items: SelectorItems,
    pub selection_mode: SelectionMode,
    pub selection: CollectionSelection,
    pub on_selection_changed: Option<SelectionEventFn>,
    pub framework: FrameworkProps,
}

pub struct ComboBoxProps {
    pub items: SelectorItems,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub editable: bool,
    pub selected_key: Option<u64>,
    pub on_selection_changed: Option<OptionalKeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct RadioButtonsProps {
    pub items: SelectorItems,
    pub header: Option<String>,
    pub selected_key: Option<u64>,
    pub max_columns: i32,
    pub on_selection_changed: Option<OptionalKeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct PivotProps {
    pub items: Vec<Element>,
    pub title: Option<String>,
    pub selected_index: Option<usize>,
    pub on_selection_changed: Option<OptionalIndexEventFn>,
    pub framework: FrameworkProps,
}

pub struct FlipViewProps {
    pub items: Vec<Element>,
    pub selected_index: Option<usize>,
    pub on_selection_changed: Option<OptionalIndexEventFn>,
    pub framework: FrameworkProps,
}

pub struct TabViewProps {
    pub items: Vec<Element>,
    pub selected_index: Option<usize>,
    pub can_reorder_tabs: bool,
    pub is_add_tab_button_visible: bool,
    pub on_selection_changed: Option<OptionalIndexEventFn>,
    pub on_close_requested: Option<KeyEventFn>,
    pub on_add_tab_button_click: Option<EventFn>,
    pub on_tabs_reordered: Option<KeysEventFn>,
    pub framework: FrameworkProps,
}

pub struct TabViewItemProps {
    pub item_key: u64,
    pub header: String,
    pub closable: bool,
}

pub struct SelectorBarProps {
    pub items: Vec<Element>,
    pub selected_key: Option<u64>,
    pub on_selection_changed: Option<OptionalKeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct SelectorBarItemProps {
    pub item_key: u64,
    pub text: String,
    pub icon: Option<Icon>,
}

pub struct BreadcrumbBarProps {
    pub items: SelectorItems,
    pub on_item_clicked: Option<KeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct AutoSuggestBoxProps {
    pub text: String,
    pub items: SelectorItems,
    pub placeholder: String,
    pub header: Option<String>,
    pub on_text_changed: Option<TextEventFn>,
    pub on_query_submitted: Option<TextEventFn>,
    pub on_suggestion_chosen: Option<KeyEventFn>,
    pub framework: FrameworkProps,
}

pub struct PivotItemProps {
    pub header: String,
}

pub struct CheckBoxProps {
    pub checked: bool,
    pub on_toggle: Option<BoolEventFn>,
    pub framework: FrameworkProps,
}

pub struct RadioButtonProps {
    pub checked: bool,
    pub group_name: Option<String>,
    pub on_toggle: Option<BoolEventFn>,
    pub framework: FrameworkProps,
}

pub struct TextBlockProps {
    pub text: String,
    pub padding: Option<Thickness>,
    pub framework: FrameworkProps,
}

pub struct TextBoxProps {
    pub text: String,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub accepts_return: bool,
    pub background: Option<Brush>,
    pub border_brush: Option<Brush>,
    pub border_thickness: Option<Thickness>,
    pub on_change: Option<TextEventFn>,
    pub framework: FrameworkProps,
}

pub struct PasswordBoxProps {
    pub password: String,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub reveal_mode: PasswordRevealMode,
    pub on_change: Option<TextEventFn>,
    pub framework: FrameworkProps,
}

pub struct ViewboxProps {
    pub stretch: Stretch,
    pub framework: FrameworkProps,
}

pub struct ScrollViewerProps {
    pub horizontal_scroll_bar_visibility: ScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollBarVisibility,
    pub on_view_changed: Option<Callback<ScrollEvent>>,
    pub framework: FrameworkProps,
}

pub struct ScrollViewProps {
    pub horizontal_scroll_bar_visibility: ScrollViewBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollViewBarVisibility,
    pub content_orientation: ScrollOrientation,
    pub on_view_changed: Option<Callback<ScrollEvent>>,
    pub framework: FrameworkProps,
}

pub struct SplitViewProps {
    pub display_mode: SplitViewDisplayMode,
    pub is_pane_open: bool,
    pub open_pane_length: f64,
    pub compact_pane_length: f64,
    pub on_pane_closed: Option<Callback<()>>,
    pub framework: FrameworkProps,
}

pub struct ExpanderProps {
    pub expanded: bool,
    pub on_expanded_changed: Option<Callback<bool>>,
    pub framework: FrameworkProps,
}

pub struct TeachingTipProps {
    pub title: String,
    pub subtitle: String,
    pub open: bool,
    pub light_dismiss: bool,
    pub action_button: Option<String>,
    pub close_button: Option<String>,
    pub on_closed: Option<Callback<()>>,
    pub on_action_button_click: Option<Callback<()>>,
}

pub struct ContentDialogProps {
    pub primary_button_text: String,
    pub secondary_button_text: String,
    pub close_button_text: String,
    pub primary_button_enabled: bool,
    pub secondary_button_enabled: bool,
    pub open: bool,
    pub on_closed: Option<Callback<ContentDialogResult>>,
}

pub struct CommandBarProps {
    pub primary: Vec<CommandBarItem>,
    pub secondary: Vec<CommandBarItem>,
    pub default_label_position: CommandBarDefaultLabelPosition,
    pub framework: FrameworkProps,
}

pub struct ImageProps {
    pub source: ImageSource,
    pub stretch: Stretch,
    pub on_load: Option<Callback<windows_core::Result<()>>>,
    pub framework: FrameworkProps,
}

pub struct AppBarButtonProps {
    pub label: String,
    pub enabled: bool,
    pub icon: Option<Icon>,
    pub on_click: EventFn,
}

pub struct AppBarToggleButtonProps {
    pub label: String,
    pub enabled: bool,
    pub checked: bool,
    pub icon: Option<Icon>,
    pub on_toggled: BoolEventFn,
}
