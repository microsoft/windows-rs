use std::rc::Rc;
use std::time::Duration;

pub(crate) const FADE_TRANSITION_TIMER_SLOT: u32 = u32::MAX;
use windows_time::{DateTime, TimeSpan};

#[cfg(feature = "canvas")]
use crate::canvas::CanvasDrawCallback;
use crate::element::IconKind;
use crate::element::props::{MenuBarItemSpec, MenuItemSpec};
use crate::element::{
    ApplicationResources, AutomationHeadingLevel, Brush, ButtonEmphasis, CalendarSelectionMode,
    CollectionSelection, Color, ColorScheme, CommandBarDefaultLabelPosition, ContentDialogResult,
    CornerRadius, DropEvent, DropTarget, ElementResources, FlyoutPlacement, FontStretch, FontStyle,
    FontWeight, GridLength, HorizontalAlignment, Icon, ImageSource, ImplicitTransitions,
    InfoBarSeverity, NavigationPaneDisplayMode, Orientation, PasswordRevealMode, PointerEvent,
    ScrollBarVisibility, ScrollEvent, ScrollOrientation, ScrollViewBarVisibility, SelectionMode,
    SelectorItem, ShapeKind, Stretch, SystemTitleBar, TextTrimming, TextWrapping, Thickness,
    TooltipPlacement, VerticalAlignment, VirtualKey, VirtualKeyModifiers, Visibility,
    WindowBackdrop, WindowConstraints, WindowIcon, WindowOverlappedPolicy, WindowPresenter,
    WindowSize, WindowTheme,
};
use crate::id::NodeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachmentShape {
    None,
    Children,
    Items,
    Content,
    ContentPane,
    HeaderContent,
}

macro_rules! define_native_catalog {
    (
        $(
            $(#[$attr:meta])*
            $kind:ident => [
                $ui:ident,
                $text:ident,
                $enabled:ident,
                $toggle:ident,
                $attachment:ident
            ],
            [$($builder:ident),*],
        )*
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum NativeKind {
            $($(#[$attr])* $kind,)*
        }

        impl NativeKind {
            #[cfg(test)]
            pub const ALL: &'static [Self] = &[
                $($(#[$attr])* Self::$kind,)*
            ];

            pub const fn supports_ui_element(self) -> bool {
                match self {
                    $($(#[$attr])* Self::$kind => define_native_catalog!(@bool $ui),)*
                }
            }

            pub const fn supports_text(self) -> bool {
                match self {
                    $($(#[$attr])* Self::$kind => define_native_catalog!(@bool $text),)*
                }
            }

            pub const fn supports_enabled(self) -> bool {
                match self {
                    $($(#[$attr])* Self::$kind => define_native_catalog!(@bool $enabled),)*
                }
            }

            pub const fn supports_toggle(self) -> bool {
                match self {
                    $($(#[$attr])* Self::$kind => define_native_catalog!(@bool $toggle),)*
                }
            }

            pub const fn attachment_shape(self) -> AttachmentShape {
                match self {
                    $($(#[$attr])* Self::$kind => AttachmentShape::$attachment,)*
                }
            }
        }
    };
    (@bool Ui) => { true };
    (@bool NoUi) => { false };
    (@bool Text) => { true };
    (@bool NoText) => { false };
    (@bool Enabled) => { true };
    (@bool NoEnabled) => { false };
    (@bool Toggle) => { true };
    (@bool NoToggle) => { false };
}

native_control_catalog!(define_native_catalog);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attachment {
    Child {
        index: usize,
    },
    Command {
        section: CommandSection,
        index: usize,
    },
    Content,
    Pane,
    Header,
    PaneFooter,
    Item {
        index: usize,
    },
    VirtualItem {
        index: usize,
        lease: u64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandSection {
    Primary,
    Secondary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationSection {
    Menu,
    Content,
    Footer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerRelation {
    ButtonFlyout,
    TeachingTipTarget,
    ToolTip,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativeUpdate {
    Resources(Box<ElementResources>),
    Visual(VisualUpdate),
    Accessibility(AccessibilityUpdate),
    Attached(AttachedUpdate),
    Input(InputUpdate),
    TextStyle(TextStyleUpdate),
    Framework(FrameworkUpdate),
    Control(ControlUpdate),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VisualUpdate {
    ImplicitTransitions(ImplicitTransitions),
    Scale(Option<f32>),
    FadeTo { opacity: f32, duration: Duration },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttachedUpdate {
    Row(Option<i32>),
    Column(Option<i32>),
    RowSpan(Option<i32>),
    ColumnSpan(Option<i32>),
    CanvasLeft(Option<f64>),
    CanvasTop(Option<f64>),
    CanvasZIndex(Option<i32>),
    RelativeAlignLeft(Option<bool>),
    RelativeAlignRight(Option<bool>),
    RelativeAlignTop(Option<bool>),
    RelativeAlignBottom(Option<bool>),
    RelativeAlignHorizontalCenter(Option<bool>),
    RelativeAlignVerticalCenter(Option<bool>),
    TooltipPlacement(Option<TooltipPlacement>),
}

impl AttachedUpdate {
    fn supports(&self, kind: NativeKind) -> bool {
        kind.supports_ui_element()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Row(_) => "set Grid.Row",
            Self::Column(_) => "set Grid.Column",
            Self::RowSpan(_) => "set Grid.RowSpan",
            Self::ColumnSpan(_) => "set Grid.ColumnSpan",
            Self::CanvasLeft(_) => "set Canvas.Left",
            Self::CanvasTop(_) => "set Canvas.Top",
            Self::CanvasZIndex(_) => "set Canvas.ZIndex",
            Self::RelativeAlignLeft(_) => "set RelativePanel.AlignLeftWithPanel",
            Self::RelativeAlignRight(_) => "set RelativePanel.AlignRightWithPanel",
            Self::RelativeAlignTop(_) => "set RelativePanel.AlignTopWithPanel",
            Self::RelativeAlignBottom(_) => "set RelativePanel.AlignBottomWithPanel",
            Self::RelativeAlignHorizontalCenter(_) => {
                "set RelativePanel.AlignHorizontalCenterWithPanel"
            }
            Self::RelativeAlignVerticalCenter(_) => {
                "set RelativePanel.AlignVerticalCenterWithPanel"
            }
            Self::TooltipPlacement(_) => "set tooltip placement",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccessibilityUpdate {
    AutomationName(String),
    AutomationId(String),
    HeadingLevel(Option<AutomationHeadingLevel>),
    HelpText(String),
}

impl AccessibilityUpdate {
    fn supports(&self, kind: NativeKind) -> bool {
        kind.supports_ui_element()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::AutomationName(_) => "set automation name",
            Self::AutomationId(_) => "set automation id",
            Self::HeadingLevel(_) => "set automation heading level",
            Self::HelpText(_) => "set automation help text",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputUpdate {
    KeyboardAccelerators(Vec<KeyboardAcceleratorSpec>),
    Pointer(PointerSubscription),
    Drop(Option<DropTarget>),
}

impl InputUpdate {
    fn supports(&self, kind: NativeKind) -> bool {
        kind.supports_ui_element()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::KeyboardAccelerators(_) => "set keyboard accelerators",
            Self::Pointer(_) => "set pointer subscription",
            Self::Drop(_) => "set drop target",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyboardAcceleratorSpec {
    pub key: VirtualKey,
    pub modifiers: VirtualKeyModifiers,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointerEvents(u16);

impl PointerEvents {
    pub const PRESSED: Self = Self(1 << 0);
    pub const MOVED: Self = Self(1 << 1);
    pub const RELEASED: Self = Self(1 << 2);
    pub const CAPTURE_LOST: Self = Self(1 << 3);
    pub const CANCELED: Self = Self(1 << 4);
    pub const ENTERED: Self = Self(1 << 5);
    pub const EXITED: Self = Self(1 << 6);
    pub const TAPPED: Self = Self(1 << 7);
    pub const RIGHT_TAPPED: Self = Self(1 << 8);

    pub const fn contains(self, value: Self) -> bool {
        self.0 & value.0 == value.0
    }
}

impl std::ops::BitOr for PointerEvents {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for PointerEvents {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointerSubscription {
    pub events: PointerEvents,
    pub capture_on_press: bool,
}

impl PointerSubscription {
    pub const fn is_empty(self) -> bool {
        self.events.0 == 0 && !self.capture_on_press
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerEventKind {
    Pressed,
    Moved,
    Released,
    CaptureLost,
    Canceled,
    Entered,
    Exited,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextStyleUpdate {
    FontFamily(Option<String>),
    Foreground(Option<Brush>),
}

impl TextStyleUpdate {
    fn supports(&self, kind: NativeKind) -> bool {
        kind.supports_text()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::FontFamily(_) => "set font family",
            Self::Foreground(_) => "set foreground",
        }
    }
}

macro_rules! scalar_property_supports {
    (all, $kind:expr) => {
        $kind.supports_ui_element()
    };
    (text, $kind:expr) => {
        $kind.supports_text()
    };
    (text_block, $kind:expr) => {
        $kind == NativeKind::TextBlock
    };
}

macro_rules! define_framework_update {
    ($(($variant:ident, $setter:ident, $ty:ty, $group:ident, $getter:ident, $capability:ident, $name:literal)),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum FrameworkUpdate {
            Width(Dimension),
            Height(Dimension),
            MinWidth(Dimension),
            MaxWidth(Dimension),
            MinHeight(Dimension),
            MaxHeight(Dimension),
            Margin(Option<Thickness>),
            Padding(Thickness),
            HorizontalAlignment(Option<HorizontalAlignment>),
            VerticalAlignment(Option<VerticalAlignment>),
            Enabled(bool),
            $($variant(Option<$ty>),)*
        }
    };
}

scalar_framework_properties!(define_framework_update);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    Default,
    Pixels(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlUpdate {
    Border(Box<BorderUpdate>),
    Shape(Box<ShapeUpdate>),
    ButtonEmphasis(ButtonEmphasis),
    FlyoutPlacement(FlyoutPlacement),
    MenuBar(Vec<MenuBarItemSpec>),
    MenuFlyout(Vec<MenuItemSpec>),
    TextBlockText(String),
    TextBox(Box<TextBoxUpdate>),
    PasswordBox(Box<PasswordBoxUpdate>),
    HyperlinkButtonNavigateUri(Option<String>),
    RepeatButton(RepeatButtonUpdate),
    ToggleChecked(bool),
    ToggleSwitch(ToggleSwitchUpdate),
    InfoBadgeValue(Option<i32>),
    InfoBar(Box<InfoBarUpdate>),
    PersonPicture(Box<PersonPictureUpdate>),
    ProgressBar(Box<ProgressBarUpdate>),
    ProgressRing(Box<ProgressRingUpdate>),
    Slider(Box<SliderUpdate>),
    NumberBox(NumberBoxUpdate),
    RatingControl(RatingControlUpdate),
    ColorPicker(ColorPickerUpdate),
    DatePicker(DatePickerUpdate),
    CalendarDatePicker(Box<CalendarDatePickerUpdate>),
    TimePicker(Box<TimePickerUpdate>),
    CalendarView(Box<CalendarViewUpdate>),
    RichEditBox(Box<RichEditBoxUpdate>),
    RichTextBlock(Box<RichTextBlockUpdate>),
    TreeView(Box<TreeViewUpdate>),
    Grid(GridUpdate),
    StackPanel(StackPanelUpdate),
    ListBox(ListBoxUpdate),
    ComboBox(Box<ComboBoxUpdate>),
    RadioButtons(RadioButtonsUpdate),
    RadioButtonGroupName(Option<String>),
    IndexSelector(i32),
    TabView(TabViewUpdate),
    TabViewItem(TabViewItemUpdate),
    SelectorBarSelection(Option<u64>),
    SelectorBarItem(SelectorBarItemUpdate),
    BreadcrumbBarItems(Rc<[SelectorItem]>),
    AutoSuggestBox(Box<AutoSuggestUpdate>),
    Pivot(PivotUpdate),
    PivotItemHeader(String),
    Collection(CollectionUpdate),
    ViewboxStretch(Stretch),
    ScrollViewer(ScrollViewerUpdate),
    ScrollView(ScrollViewUpdate),
    SplitView(SplitViewUpdate),
    Expander(ExpanderUpdate),
    TeachingTip(TeachingTipUpdate),
    TitleBar(Box<TitleBarUpdate>),
    ContentDialog(Box<ContentDialogUpdate>),
    CommandBar(CommandBarDefaultLabelPosition),
    CompositionHost(Box<CompositionHostUpdate>),
    #[cfg(feature = "canvas")]
    CanvasImage(CanvasUpdate),
    #[cfg(feature = "canvas")]
    SwapChainCanvas(Box<SwapChainCanvasUpdate>),
    #[cfg(feature = "canvas")]
    SwapChainHost(Box<SwapChainHostUpdate>),
    #[cfg(feature = "webview")]
    WebViewHost(WebViewHostUpdate),
    Image(Box<ImageUpdate>),
    Icon(Box<Icon>),
    NavigationView(NavigationUpdate),
    NavigationViewItem(Box<NavigationViewItemUpdate>),
    AppBarButton(Box<AppBarButtonUpdate>),
    AppBarToggleButton(Box<AppBarToggleButtonUpdate>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompositionHostUpdate {
    Initialize {
        factory: crate::composition::CompositionFactory,
        layout: crate::composition::CompositionLayoutCallback,
    },
    LayoutCallback(crate::composition::CompositionLayoutCallback),
    Action(crate::composition::CompositionAction),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextBoxUpdate {
    Text(String),
    Header(Option<String>),
    Placeholder(Option<String>),
    AcceptsReturn(bool),
    Chrome(Box<ControlChromeUpdate>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PasswordBoxUpdate {
    Password(String),
    Header(Option<String>),
    Placeholder(Option<String>),
    RevealMode(PasswordRevealMode),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToggleSwitchUpdate {
    On(bool),
    Content(Box<ToggleSwitchContentUpdate>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressBarUpdate {
    Range(RangeState),
    Indeterminate(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressRingUpdate {
    Range(RangeState),
    Active(bool),
    Indeterminate(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum SliderUpdate {
    Range(RangeState),
    Orientation(Orientation),
    Step(f64),
    Header(Option<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberBoxUpdate {
    Bounds { minimum: f64, maximum: f64 },
    Value(Option<f64>),
    Header(Option<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RatingControlUpdate {
    Max(i32),
    Placeholder(Option<f64>),
    Caption(String),
    ReadOnly(bool),
    Value(Option<f64>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerUpdate {
    Color(Color),
    AlphaEnabled(bool),
    HexInputVisible(bool),
    SliderVisible(bool),
    ChannelInputVisible(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DatePickerUpdate {
    Date(Option<DateTime>),
    Header(Option<String>),
    DayVisible(bool),
    MonthVisible(bool),
    YearVisible(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalendarDatePickerUpdate {
    Date(Option<DateTime>),
    Header(Option<String>),
    Placeholder(Option<String>),
    TodayHighlighted(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComboBoxUpdate {
    Items(Rc<[SelectorItem]>),
    Selection(Option<u64>),
    Header(Option<String>),
    Placeholder(Option<String>),
    Editable(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RadioButtonsUpdate {
    Items(Rc<[SelectorItem]>),
    Selection(Option<u64>),
    Header(Option<String>),
    MaxColumns(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NavigationUpdate {
    Properties(Box<NavigationViewUpdate>),
    Selection(Option<u64>),
}

#[cfg(feature = "canvas")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SwapChainCanvasUpdate {
    Canvas(CanvasUpdate),
    Continuous(bool),
}

#[cfg(feature = "canvas")]
#[derive(Clone, Debug, PartialEq)]
pub enum SwapChainHostUpdate {
    Initialize {
        factory: crate::canvas::SwapChainHostFactory,
        layout: crate::canvas::SwapChainHostLayoutCallback,
        frame: crate::canvas::SwapChainHostFrameCallback,
        continuous: bool,
    },
    Props {
        layout: crate::canvas::SwapChainHostLayoutCallback,
        frame: crate::canvas::SwapChainHostFrameCallback,
        continuous: bool,
    },
    Action(crate::canvas::SwapChainHostAction),
}

#[cfg(feature = "webview")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebViewHostUpdate {
    Source(Option<String>),
    Action(crate::webview::WebViewAction),
}

impl TextBoxUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Text(_) => "set text",
            Self::Header(_) => "set text box header",
            Self::Placeholder(_) => "set text box placeholder",
            Self::AcceptsReturn(_) => "set text box accepts return",
            Self::Chrome(_) => "set control chrome",
        }
    }
}

impl PasswordBoxUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Password(_) => "set password",
            Self::Header(_) => "set password box header",
            Self::Placeholder(_) => "set password box placeholder",
            Self::RevealMode(_) => "set password reveal mode",
        }
    }
}

impl ToggleSwitchUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::On(_) => "set on",
            Self::Content(_) => "set ToggleSwitch content",
        }
    }
}

impl ProgressBarUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Range(_) => "set ProgressBar range",
            Self::Indeterminate(_) => "set ProgressBar indeterminate state",
        }
    }
}

impl ProgressRingUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Range(_) => "set ProgressRing range",
            Self::Active(_) => "set ProgressRing active state",
            Self::Indeterminate(_) => "set ProgressRing indeterminate state",
        }
    }
}

impl SliderUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Range(_) => "set Slider range",
            Self::Orientation(_) => "set Slider orientation",
            Self::Step(_) => "set Slider step",
            Self::Header(_) => "set Slider header",
        }
    }
}

impl NumberBoxUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Bounds { .. } => "set NumberBox bounds",
            Self::Value(_) => "set NumberBox value",
            Self::Header(_) => "set NumberBox header",
        }
    }
}

impl RatingControlUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Max(_) => "set maximum rating",
            Self::Placeholder(_) => "set rating placeholder",
            Self::Caption(_) => "set rating caption",
            Self::ReadOnly(_) => "set rating read-only state",
            Self::Value(_) => "set rating value",
        }
    }
}

impl ColorPickerUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Color(_) => "set ColorPicker color",
            Self::AlphaEnabled(_) => "set ColorPicker alpha state",
            Self::HexInputVisible(_) => "set ColorPicker hex input visibility",
            Self::SliderVisible(_) => "set ColorPicker slider visibility",
            Self::ChannelInputVisible(_) => "set ColorPicker channel input visibility",
        }
    }
}

impl DatePickerUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Date(_) => "set DatePicker date",
            Self::Header(_) => "set DatePicker header",
            Self::DayVisible(_) => "set DatePicker day visibility",
            Self::MonthVisible(_) => "set DatePicker month visibility",
            Self::YearVisible(_) => "set DatePicker year visibility",
        }
    }
}

impl CalendarDatePickerUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Date(_) => "set CalendarDatePicker date",
            Self::Header(_) => "set CalendarDatePicker header",
            Self::Placeholder(_) => "set CalendarDatePicker placeholder",
            Self::TodayHighlighted(_) => "set CalendarDatePicker today highlight",
        }
    }
}

impl ComboBoxUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Items(_) => "set ComboBox items",
            Self::Selection(_) => "set ComboBox selection",
            Self::Header(_) => "set ComboBox header",
            Self::Placeholder(_) => "set ComboBox placeholder",
            Self::Editable(_) => "set ComboBox editable",
        }
    }
}

impl RadioButtonsUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Items(_) => "set RadioButtons items",
            Self::Selection(_) => "set RadioButtons selection",
            Self::Header(_) => "set RadioButtons header",
            Self::MaxColumns(_) => "set RadioButtons max columns",
        }
    }
}

impl NavigationUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Properties(_) => "update NavigationView",
            Self::Selection(_) => "set NavigationView selection",
        }
    }
}

#[cfg(feature = "canvas")]
impl SwapChainCanvasUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Canvas(CanvasUpdate::Props { .. } | CanvasUpdate::Rebind { .. }) => {
                "update swap-chain canvas"
            }
            Self::Canvas(CanvasUpdate::Invalidate(_)) => "invalidate swap-chain canvas",
            Self::Continuous(_) => "set swap-chain canvas render mode",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlChromeUpdate {
    pub background: Option<Brush>,
    pub border_brush: Option<Brush>,
    pub border_thickness: Option<Thickness>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeUpdate {
    pub kind: ShapeKind,
    pub fill: Option<Brush>,
    pub stroke: Option<Brush>,
    pub stroke_thickness: Option<f64>,
    pub corner_radius: Option<f64>,
    pub line: [f64; 4],
}

impl ControlUpdate {
    fn supports(&self, kind: NativeKind) -> bool {
        match self {
            Self::Border(_) => kind == NativeKind::Border,
            Self::Shape(update) => kind == update.kind.native_kind(),
            Self::ButtonEmphasis(_) => kind == NativeKind::Button,
            Self::FlyoutPlacement(_) => {
                matches!(
                    kind,
                    NativeKind::Flyout | NativeKind::MenuFlyout | NativeKind::CommandBarFlyout
                )
            }
            Self::MenuBar(_) => kind == NativeKind::MenuBar,
            Self::MenuFlyout(_) => kind == NativeKind::MenuFlyout,
            Self::TextBlockText(_) => kind == NativeKind::TextBlock,
            Self::TextBox(_) => kind == NativeKind::TextBox,
            Self::PasswordBox(_) => kind == NativeKind::PasswordBox,
            Self::HyperlinkButtonNavigateUri(_) => kind == NativeKind::HyperlinkButton,
            Self::RepeatButton(_) => kind == NativeKind::RepeatButton,
            Self::ToggleChecked(_) => kind.supports_toggle(),
            Self::ToggleSwitch(_) => kind == NativeKind::ToggleSwitch,
            Self::InfoBadgeValue(_) => kind == NativeKind::InfoBadge,
            Self::InfoBar(_) => kind == NativeKind::InfoBar,
            Self::PersonPicture(_) => kind == NativeKind::PersonPicture,
            Self::ProgressBar(_) => kind == NativeKind::ProgressBar,
            Self::ProgressRing(_) => kind == NativeKind::ProgressRing,
            Self::Slider(_) => kind == NativeKind::Slider,
            Self::NumberBox(_) => kind == NativeKind::NumberBox,
            Self::RatingControl(_) => kind == NativeKind::RatingControl,
            Self::ColorPicker(_) => kind == NativeKind::ColorPicker,
            Self::DatePicker(_) => kind == NativeKind::DatePicker,
            Self::CalendarDatePicker(_) => kind == NativeKind::CalendarDatePicker,
            Self::TimePicker(_) => kind == NativeKind::TimePicker,
            Self::CalendarView(_) => kind == NativeKind::CalendarView,
            Self::RichEditBox(_) => kind == NativeKind::RichEditBox,
            Self::RichTextBlock(_) => kind == NativeKind::RichTextBlock,
            Self::TreeView(_) => kind == NativeKind::TreeView,
            Self::Grid(_) => kind == NativeKind::Grid,
            Self::StackPanel(_) => kind == NativeKind::StackPanel,
            Self::ListBox(_) => kind == NativeKind::ListBox,
            Self::ComboBox(_) => kind == NativeKind::ComboBox,
            Self::RadioButtons(_) => kind == NativeKind::RadioButtons,
            Self::RadioButtonGroupName(_) => kind == NativeKind::RadioButton,
            Self::IndexSelector(_) => {
                matches!(
                    kind,
                    NativeKind::FlipView | NativeKind::Pivot | NativeKind::TabView
                )
            }
            Self::TabView(_) => kind == NativeKind::TabView,
            Self::TabViewItem(_) => kind == NativeKind::TabViewItem,
            Self::SelectorBarSelection(_) => kind == NativeKind::SelectorBar,
            Self::SelectorBarItem(_) => kind == NativeKind::SelectorBarItem,
            Self::BreadcrumbBarItems(_) => kind == NativeKind::BreadcrumbBar,
            Self::AutoSuggestBox(_) => kind == NativeKind::AutoSuggestBox,
            Self::Pivot(_) => kind == NativeKind::Pivot,
            Self::PivotItemHeader(_) => kind == NativeKind::PivotItem,
            Self::Collection(_) => matches!(kind, NativeKind::ListView | NativeKind::GridView),
            Self::ViewboxStretch(_) => kind == NativeKind::Viewbox,
            Self::ScrollViewer(_) => kind == NativeKind::ScrollViewer,
            Self::ScrollView(_) => kind == NativeKind::ScrollView,
            Self::SplitView(_) => kind == NativeKind::SplitView,
            Self::Expander(_) => kind == NativeKind::Expander,
            Self::TeachingTip(_) => kind == NativeKind::TeachingTip,
            Self::TitleBar(_) => kind == NativeKind::TitleBar,
            Self::ContentDialog(_) => kind == NativeKind::ContentDialog,
            Self::CommandBar(_) => kind == NativeKind::CommandBar,
            Self::CompositionHost(_) => kind == NativeKind::CompositionHost,
            #[cfg(feature = "canvas")]
            Self::CanvasImage(_) => kind == NativeKind::CanvasImage,
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(_) => kind == NativeKind::SwapChainCanvas,
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(_) => kind == NativeKind::SwapChainHost,
            #[cfg(feature = "webview")]
            Self::WebViewHost(_) => kind == NativeKind::WebViewHost,
            Self::Image(_) => kind == NativeKind::Image,
            Self::Icon(icon) => match icon.kind() {
                IconKind::Symbol(_) => kind == NativeKind::SymbolIcon,
                IconKind::Font { .. } => kind == NativeKind::FontIcon,
                IconKind::Bitmap { .. } => kind == NativeKind::BitmapIcon,
                IconKind::Image(_) => kind == NativeKind::ImageIcon,
                IconKind::Path(_) => kind == NativeKind::PathIcon,
            },
            Self::NavigationView(_) => kind == NativeKind::NavigationView,
            Self::NavigationViewItem(_) => kind == NativeKind::NavigationViewItem,
            Self::AppBarButton(_) => kind == NativeKind::AppBarButton,
            Self::AppBarToggleButton(_) => kind == NativeKind::AppBarToggleButton,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Border(update) => match update.as_ref() {
                BorderUpdate::Background(_) => "set Border background",
                BorderUpdate::BorderBrush(_) => "set Border border brush",
                BorderUpdate::BorderThickness(_) => "set Border border thickness",
                BorderUpdate::CornerRadius(_) => "set Border corner radius",
                BorderUpdate::Padding(_) => "set Border padding",
            },
            Self::Shape(_) => "set shape properties",
            Self::ButtonEmphasis(_) => "set Button emphasis",
            Self::FlyoutPlacement(_) => "set Flyout placement",
            Self::MenuBar(_) => "set MenuBar items",
            Self::MenuFlyout(_) => "set MenuFlyout items",
            Self::TextBlockText(_) => "set text",
            Self::TextBox(update) => update.description(),
            Self::PasswordBox(update) => update.description(),
            Self::HyperlinkButtonNavigateUri(_) => "set HyperlinkButton.NavigateUri",
            Self::RepeatButton(RepeatButtonUpdate::Delay(_)) => "set RepeatButton.Delay",
            Self::RepeatButton(RepeatButtonUpdate::Interval(_)) => "set RepeatButton.Interval",
            Self::ToggleChecked(_) => "set checked",
            Self::ToggleSwitch(update) => update.description(),
            Self::InfoBadgeValue(_) => "set InfoBadge value",
            Self::InfoBar(_) => "update InfoBar",
            Self::PersonPicture(_) => "update PersonPicture",
            Self::ProgressBar(update) => update.description(),
            Self::ProgressRing(update) => update.description(),
            Self::Slider(update) => update.description(),
            Self::NumberBox(update) => update.description(),
            Self::RatingControl(update) => update.description(),
            Self::ColorPicker(update) => update.description(),
            Self::DatePicker(update) => update.description(),
            Self::CalendarDatePicker(update) => update.description(),
            Self::TimePicker(_) => "set TimePicker properties",
            Self::CalendarView(_) => "set CalendarView properties",
            Self::RichEditBox(_) => "set RichEditBox properties",
            Self::RichTextBlock(_) => "set RichTextBlock document",
            Self::TreeView(update) => match update.as_ref() {
                TreeViewUpdate::Nodes(_) => "set TreeView nodes",
                TreeViewUpdate::ExpandedChanged(_) => "set TreeView expanded-changed subscription",
            },
            Self::Grid(GridUpdate::Columns(_)) => "set Grid columns",
            Self::Grid(GridUpdate::Rows(_)) => "set Grid rows",
            Self::Grid(GridUpdate::ColumnSpacing(_)) => "set Grid column spacing",
            Self::Grid(GridUpdate::RowSpacing(_)) => "set Grid row spacing",
            Self::StackPanel(StackPanelUpdate::Orientation(_)) => "set StackPanel orientation",
            Self::StackPanel(StackPanelUpdate::Spacing(_)) => "set StackPanel spacing",
            Self::ListBox(ListBoxUpdate::Items(_)) => "set ListBox items",
            Self::ListBox(ListBoxUpdate::SelectionMode(_)) => "set ListBox selection mode",
            Self::ListBox(ListBoxUpdate::Selection(_)) => "set ListBox selection",
            Self::ComboBox(update) => update.description(),
            Self::RadioButtons(update) => update.description(),
            Self::RadioButtonGroupName(_) => "set RadioButton group name",
            Self::IndexSelector(_) => "set selector selected index",
            Self::TabView(TabViewUpdate::CanReorderTabs(_)) => "set TabView reorder state",
            Self::TabView(TabViewUpdate::IsAddTabButtonVisible(_)) => {
                "set TabView add-button visibility"
            }
            Self::TabViewItem(TabViewItemUpdate::Header(_)) => "set TabViewItem header",
            Self::TabViewItem(TabViewItemUpdate::Closable(_)) => "set TabViewItem closable state",
            Self::TabViewItem(TabViewItemUpdate::Key(_)) => "set TabViewItem key",
            Self::SelectorBarSelection(_) => "set SelectorBar selected key",
            Self::SelectorBarItem(SelectorBarItemUpdate::Key(_)) => "set SelectorBarItem key",
            Self::SelectorBarItem(SelectorBarItemUpdate::Text(_)) => "set SelectorBarItem text",
            Self::SelectorBarItem(SelectorBarItemUpdate::Icon(_)) => "set SelectorBarItem icon",
            Self::BreadcrumbBarItems(_) => "set BreadcrumbBar items",
            Self::AutoSuggestBox(update) => update.description(),
            Self::Pivot(PivotUpdate::Title(_)) => "set Pivot title",
            Self::PivotItemHeader(_) => "set PivotItem header",
            Self::Collection(CollectionUpdate::ItemCount(_)) => "set collection item count",
            Self::Collection(CollectionUpdate::ItemKeys(_)) => "set collection item keys",
            Self::Collection(CollectionUpdate::SelectionMode(_)) => "set collection selection mode",
            Self::Collection(CollectionUpdate::Selection(_)) => "set collection selection",
            Self::Collection(CollectionUpdate::SelectionDisplayOnly(_)) => {
                "set collection display-only selection"
            }
            Self::Collection(CollectionUpdate::ItemClickEnabled(_)) => {
                "set collection item-click state"
            }
            Self::Collection(CollectionUpdate::CanReorderItems(_)) => {
                "set collection reorder state"
            }
            Self::ViewboxStretch(_) => "set Viewbox.Stretch",
            Self::ScrollViewer(ScrollViewerUpdate::HorizontalScrollBarVisibility(_)) => {
                "set ScrollViewer horizontal scroll bar visibility"
            }
            Self::ScrollViewer(ScrollViewerUpdate::VerticalScrollBarVisibility(_)) => {
                "set ScrollViewer vertical scroll bar visibility"
            }
            Self::ScrollViewer(ScrollViewerUpdate::ViewChanged(_)) => {
                "set ScrollViewer view-changed subscription"
            }
            Self::ScrollView(ScrollViewUpdate::HorizontalScrollBarVisibility(_)) => {
                "set ScrollView horizontal scroll bar visibility"
            }
            Self::ScrollView(ScrollViewUpdate::VerticalScrollBarVisibility(_)) => {
                "set ScrollView vertical scroll bar visibility"
            }
            Self::ScrollView(ScrollViewUpdate::ContentOrientation(_)) => {
                "set ScrollView content orientation"
            }
            Self::ScrollView(ScrollViewUpdate::ViewChanged(_)) => {
                "set ScrollView view-changed subscription"
            }
            Self::SplitView(SplitViewUpdate::DisplayMode(_)) => "set SplitView.DisplayMode",
            Self::SplitView(SplitViewUpdate::IsPaneOpen(_)) => "set SplitView.IsPaneOpen",
            Self::SplitView(SplitViewUpdate::OpenPaneLength(_)) => "set SplitView.OpenPaneLength",
            Self::SplitView(SplitViewUpdate::CompactPaneLength(_)) => {
                "set SplitView.CompactPaneLength"
            }
            Self::SplitView(SplitViewUpdate::PaneClosed(_)) => {
                "set SplitView pane-closed subscription"
            }
            Self::Expander(ExpanderUpdate::Expanded(_)) => "set Expander.IsExpanded",
            Self::Expander(ExpanderUpdate::ExpandedChanged(_)) => {
                "set Expander expanded-changed subscription"
            }
            Self::TeachingTip(TeachingTipUpdate::Title(_)) => "set TeachingTip.Title",
            Self::TeachingTip(TeachingTipUpdate::Subtitle(_)) => "set TeachingTip.Subtitle",
            Self::TeachingTip(TeachingTipUpdate::Open(_)) => "set TeachingTip.IsOpen",
            Self::TeachingTip(TeachingTipUpdate::LightDismiss(_)) => {
                "set TeachingTip light-dismiss state"
            }
            Self::TeachingTip(TeachingTipUpdate::ActionButton(_)) => {
                "set TeachingTip action button"
            }
            Self::TeachingTip(TeachingTipUpdate::CloseButton(_)) => "set TeachingTip close button",
            Self::TeachingTip(TeachingTipUpdate::ActionButtonClick(_)) => {
                "set TeachingTip action-button subscription"
            }
            Self::TitleBar(update) => match update.as_ref() {
                TitleBarUpdate::Title(_) => "set TitleBar.Title",
                TitleBarUpdate::Subtitle(_) => "set TitleBar.Subtitle",
                TitleBarUpdate::BackButtonVisible(_) => "set TitleBar.IsBackButtonVisible",
                TitleBarUpdate::BackButtonEnabled(_) => "set TitleBar.IsBackButtonEnabled",
                TitleBarUpdate::PaneToggleButtonVisible(_) => {
                    "set TitleBar.IsPaneToggleButtonVisible"
                }
            },
            Self::ContentDialog(_) => "update ContentDialog",
            Self::CommandBar(_) => "set CommandBar default label position",
            Self::CompositionHost(update) => match update.as_ref() {
                CompositionHostUpdate::Initialize { .. } => "initialize Composition host",
                CompositionHostUpdate::LayoutCallback(_) => {
                    "update Composition host layout callback"
                }
                CompositionHostUpdate::Action(_) => "run Composition host action",
            },
            #[cfg(feature = "canvas")]
            Self::CanvasImage(CanvasUpdate::Props { .. } | CanvasUpdate::Rebind { .. }) => {
                "update canvas image"
            }
            #[cfg(feature = "canvas")]
            Self::CanvasImage(CanvasUpdate::Invalidate(_)) => "invalidate canvas image",
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(update) => update.description(),
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(update) => match update.as_ref() {
                SwapChainHostUpdate::Initialize { .. } => "initialize swap-chain host",
                SwapChainHostUpdate::Props { .. } => "update swap-chain host",
                SwapChainHostUpdate::Action(_) => "run swap-chain host action",
            },
            #[cfg(feature = "webview")]
            Self::WebViewHost(update) => match update {
                WebViewHostUpdate::Source(_) => "set WebView source",
                WebViewHostUpdate::Action(_) => "run WebView action",
            },
            Self::Image(_) => "update Image",
            Self::Icon(_) => "update icon",
            Self::NavigationView(update) => update.description(),
            Self::NavigationViewItem(_) => "update NavigationViewItem",
            Self::AppBarButton(_) => "update AppBarButton",
            Self::AppBarToggleButton(_) => "update AppBarToggleButton",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimePickerUpdate {
    pub time: Option<TimeSpan>,
    pub header: Option<String>,
    pub minute_increment: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarViewUpdate {
    pub selected_dates: Rc<[DateTime]>,
    pub selection_mode: CalendarSelectionMode,
    pub today_highlighted: bool,
    pub group_label_visible: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RichEditBoxUpdate {
    pub text: String,
    pub header: Option<String>,
    pub placeholder: Option<String>,
    pub read_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RichTextBlockUpdate {
    pub paragraphs: Rc<[crate::element::RichTextParagraph]>,
    pub font_size: Option<f64>,
    pub selectable: bool,
    pub wrap: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavigationViewUpdate {
    pub header: Option<String>,
    pub pane_title: Option<String>,
    pub settings_visible: bool,
    pub pane_toggle_visible: bool,
    pub pane_open: bool,
    pub open_pane_length: f64,
    pub pane_display_mode: NavigationPaneDisplayMode,
    pub selection_feedback: bool,
    pub pane_feedback: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeViewUpdate {
    Nodes(Rc<[crate::element::TreeNode]>),
    ExpandedChanged(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationViewItemUpdate {
    pub item_key: u64,
    pub label: String,
    pub icon: Option<Icon>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackPanelUpdate {
    Orientation(Orientation),
    Spacing(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderUpdate {
    Background(Option<Brush>),
    BorderBrush(Option<Brush>),
    BorderThickness(Option<Thickness>),
    CornerRadius(Option<CornerRadius>),
    Padding(Option<Thickness>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToggleSwitchContentUpdate {
    pub header: Option<String>,
    pub on_content: Option<String>,
    pub off_content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfoBarUpdate {
    pub title: String,
    pub message: String,
    pub severity: InfoBarSeverity,
    pub open: bool,
    pub closable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersonPictureUpdate {
    pub display_name: Option<String>,
    pub initials: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CollectionUpdate {
    ItemCount(usize),
    ItemKeys(Rc<[u64]>),
    SelectionMode(SelectionMode),
    Selection(CollectionSelection),
    SelectionDisplayOnly(bool),
    ItemClickEnabled(bool),
    CanReorderItems(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListBoxUpdate {
    Items(Rc<[SelectorItem]>),
    SelectionMode(SelectionMode),
    Selection(CollectionSelection),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PivotUpdate {
    Title(Option<String>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabViewUpdate {
    CanReorderTabs(bool),
    IsAddTabButtonVisible(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabViewItemUpdate {
    Key(u64),
    Header(String),
    Closable(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorBarItemUpdate {
    Key(u64),
    Text(String),
    Icon(Option<Rc<Icon>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutoSuggestUpdate {
    Text(String),
    Items(Rc<[SelectorItem]>),
    Header(Option<String>),
    Placeholder(String),
}

impl AutoSuggestUpdate {
    fn description(&self) -> &'static str {
        match self {
            Self::Text(_) => "set AutoSuggestBox text",
            Self::Items(_) => "set AutoSuggestBox items",
            Self::Header(_) => "set AutoSuggestBox header",
            Self::Placeholder(_) => "set AutoSuggestBox placeholder",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GridUpdate {
    Columns(Box<[GridLength]>),
    Rows(Box<[GridLength]>),
    ColumnSpacing(f64),
    RowSpacing(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepeatButtonUpdate {
    Delay(i32),
    Interval(i32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeState {
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollViewerUpdate {
    HorizontalScrollBarVisibility(ScrollBarVisibility),
    VerticalScrollBarVisibility(ScrollBarVisibility),
    ViewChanged(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollViewUpdate {
    HorizontalScrollBarVisibility(ScrollViewBarVisibility),
    VerticalScrollBarVisibility(ScrollViewBarVisibility),
    ContentOrientation(ScrollOrientation),
    ViewChanged(bool),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitViewUpdate {
    DisplayMode(crate::element::SplitViewDisplayMode),
    IsPaneOpen(bool),
    OpenPaneLength(f64),
    CompactPaneLength(f64),
    PaneClosed(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpanderUpdate {
    Expanded(bool),
    ExpandedChanged(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TeachingTipUpdate {
    Title(String),
    Subtitle(String),
    Open(bool),
    LightDismiss(bool),
    ActionButton(Option<String>),
    CloseButton(Option<String>),
    ActionButtonClick(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TitleBarUpdate {
    Title(Option<String>),
    Subtitle(Option<String>),
    BackButtonVisible(bool),
    BackButtonEnabled(bool),
    PaneToggleButtonVisible(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentDialogUpdate {
    pub primary_button_text: String,
    pub secondary_button_text: String,
    pub close_button_text: String,
    pub primary_button_enabled: bool,
    pub secondary_button_enabled: bool,
    pub open: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppBarButtonUpdate {
    pub label: String,
    pub enabled: bool,
    pub icon: Option<Icon>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppBarToggleButtonUpdate {
    pub label: String,
    pub enabled: bool,
    pub checked: bool,
    pub icon: Option<Icon>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageUpdate {
    pub source: ImageSource,
    pub source_revision: u64,
    pub source_changed: bool,
    pub stretch: Stretch,
}

#[cfg(feature = "canvas")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CanvasUpdate {
    Props {
        draw: CanvasDrawCallback,
        invalidation_revision: u64,
    },
    Rebind {
        draw: CanvasDrawCallback,
        invalidation_revision: u64,
    },
    Invalidate(u64),
}

macro_rules! define_scalar_native_update {
    ($(($variant:ident, $setter:ident, $ty:ty, $group:ident, $getter:ident, $capability:ident, $name:literal)),* $(,)?) => {
        impl FrameworkUpdate {
            fn supports(&self, kind: NativeKind) -> bool {
                match self {
                    Self::Width(_)
                    | Self::Height(_)
                    | Self::MinWidth(_)
                    | Self::MaxWidth(_)
                    | Self::MinHeight(_)
                    | Self::MaxHeight(_)
                    | Self::Margin(_)
                    | Self::HorizontalAlignment(_)
                    | Self::VerticalAlignment(_) => kind.supports_ui_element(),
                    Self::Padding(_) => {
                        matches!(kind, NativeKind::StackPanel | NativeKind::TextBlock)
                    }
                    Self::Enabled(_) => kind.supports_enabled(),
                    $(Self::$variant(_) => scalar_property_supports!($capability, kind),)*
                }
            }

            fn name(&self) -> &'static str {
                match self {
                    Self::Width(_) => "set width",
                    Self::Height(_) => "set height",
                    Self::MinWidth(_) => "set minimum width",
                    Self::MaxWidth(_) => "set maximum width",
                    Self::MinHeight(_) => "set minimum height",
                    Self::MaxHeight(_) => "set maximum height",
                    Self::Margin(_) => "set margin",
                    Self::Padding(_) => "set padding",
                    Self::HorizontalAlignment(_) => "set horizontal alignment",
                    Self::VerticalAlignment(_) => "set vertical alignment",
                    Self::Enabled(_) => "set enabled",
                    $(Self::$variant(_) => $name,)*
                }
            }
        }

        impl NativeUpdate {
            pub fn supports(&self, kind: NativeKind) -> bool {
                match self {
                    Self::Resources(_) => kind.supports_ui_element(),
                    Self::Visual(_) => kind.supports_ui_element(),
                    Self::Accessibility(update) => update.supports(kind),
                    Self::Attached(update) => update.supports(kind),
                    Self::Input(update) => update.supports(kind),
                    Self::TextStyle(update) => update.supports(kind),
                    Self::Framework(update) => update.supports(kind),
                    Self::Control(update) => update.supports(kind),
                }
            }

            pub fn name(&self) -> &'static str {
                match self {
                    Self::Resources(_) => "set element resources",
                    Self::Visual(VisualUpdate::ImplicitTransitions(_)) => {
                        "set implicit transitions"
                    }
                    Self::Visual(VisualUpdate::Scale(_)) => "set scale",
                    Self::Visual(VisualUpdate::FadeTo { .. }) => "fade element",
                    Self::Accessibility(update) => update.name(),
                    Self::Attached(update) => update.name(),
                    Self::Input(update) => update.name(),
                    Self::TextStyle(update) => update.name(),
                    Self::Framework(update) => update.name(),
                    Self::Control(update) => update.name(),
                }
            }
        }
    };
}

scalar_framework_properties!(define_scalar_native_update);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowCreate {
    pub title: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowConstraintsUpdate {
    min_width: f32,
    min_height: f32,
    max_width: f32,
    max_height: f32,
}

impl From<WindowConstraints> for WindowConstraintsUpdate {
    fn from(value: WindowConstraints) -> Self {
        let encode = |value: Option<f64>| value.map_or(f32::NAN, |value| value as f32);
        Self {
            min_width: encode(value.min_width),
            min_height: encode(value.min_height),
            max_width: encode(value.max_width),
            max_height: encode(value.max_height),
        }
    }
}

impl WindowConstraintsUpdate {
    pub(crate) fn value(self) -> WindowConstraints {
        let decode = |value: f32| (!value.is_nan()).then_some(value as f64);
        WindowConstraints {
            min_width: decode(self.min_width),
            min_height: decode(self.min_height),
            max_width: decode(self.max_width),
            max_height: decode(self.max_height),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WindowUpdate {
    Title(String),
    Backdrop(Option<WindowBackdrop>),
    Icon(WindowIcon),
    Theme(WindowTheme),
    TitleBar(Box<SystemTitleBar>),
    BindTitleBar(NodeId),
    UnbindTitleBar,
    Overlapped(WindowOverlappedPolicy),
    ClientSize(WindowSize),
    Constraints(WindowConstraintsUpdate),
    Presenter(WindowPresenter),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ApplicationUpdate {
    Resources(Box<ApplicationResources>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    StartTimer(TimerSpec),
    StopTimer {
        owner: NodeId,
        slot: u32,
        revision: u64,
    },
    UpdateApplication {
        id: NodeId,
        update: ApplicationUpdate,
    },
    CreateWindow {
        id: NodeId,
        create: WindowCreate,
    },
    SetWindowContent {
        window: NodeId,
        content: NodeId,
    },
    SetWindowOwner {
        owner: NodeId,
        child: NodeId,
    },
    UpdateWindow {
        id: NodeId,
        update: WindowUpdate,
    },
    ActivateWindow {
        id: NodeId,
    },
    FocusElement {
        id: NodeId,
    },
    CloseWindow {
        id: NodeId,
    },
    Create {
        id: NodeId,
        kind: NativeKind,
    },
    Attach {
        parent: NodeId,
        child: NodeId,
        attachment: Attachment,
    },
    Detach {
        parent: NodeId,
        child: NodeId,
    },
    BindOwner {
        owner: NodeId,
        accessory: NodeId,
        relation: OwnerRelation,
    },
    UnbindOwner {
        owner: NodeId,
        accessory: NodeId,
        relation: OwnerRelation,
    },
    Move {
        parent: NodeId,
        child: NodeId,
        index: usize,
    },
    RunDeferred {
        target: NodeId,
        window: Option<NodeId>,
        revision: u64,
        action: DeferredAction,
    },
    #[cfg(feature = "canvas")]
    ApplyCanvasImageLayout {
        target: NodeId,
        width: f32,
        height: f32,
        scale: f32,
    },
    #[cfg(feature = "canvas")]
    RunCanvasImageFrame {
        target: NodeId,
    },
    #[cfg(feature = "canvas")]
    ApplyCanvasLayout {
        target: NodeId,
        width: f32,
        height: f32,
        scale_x: f32,
        scale_y: f32,
    },
    ApplyCompositionLayout {
        target: NodeId,
        width: f32,
        height: f32,
        rasterization_scale: f32,
    },
    #[cfg(feature = "canvas")]
    RunCanvasFrame {
        target: NodeId,
    },
    #[cfg(feature = "canvas")]
    ApplySwapChainHostLayout {
        target: NodeId,
        layout: Box<crate::canvas::SwapChainHostLayout>,
    },
    #[cfg(feature = "canvas")]
    RunSwapChainHostFrame {
        target: NodeId,
    },
    #[cfg(feature = "webview")]
    FinishWebViewInitialization {
        target: NodeId,
        revision: u64,
    },
    Destroy {
        id: NodeId,
    },
    Update {
        id: NodeId,
        update: NativeUpdate,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimerSpec {
    pub owner: NodeId,
    pub slot: u32,
    pub revision: u64,
    pub interval: Duration,
    pub repeating: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeferredAction {
    ContentDialogOpen,
    TeachingTipOpen,
    RadioButtonsSelection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeEventClass {
    ControlledFeedback,
    NotificationOnly,
    CancelableRequest,
    ClosureFailureSynchronization,
    InternalRuntime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeEventCompatibility {
    TimerOwner,
    Window,
    Click,
    Menu,
    Text,
    Password,
    Toggle,
    Slider,
    OptionalValue,
    Color,
    Date,
    Time,
    Dates,
    Framework,
    Scroll,
    SplitView,
    NavigationView,
    Expander,
    TreeView,
    TeachingTip,
    InfoBar,
    TitleBar,
    Flyout,
    ContentDialog,
    Image,
    CompositionHost,
    #[cfg(feature = "canvas")]
    CanvasImage,
    #[cfg(feature = "canvas")]
    SwapChainCanvas,
    #[cfg(feature = "canvas")]
    SwapChainHost,
    #[cfg(feature = "webview")]
    WebViewHost,
    DeferredContentDialog,
    DeferredTeachingTip,
    DeferredRadioButtons,
    ItemInvocation,
    AutoSuggestBox,
    CollectionSelection,
    IndexSelection,
    TabView,
    VirtualCollection,
    SingleSelection,
    VirtualHost,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NativeEvent {
    TimerFired {
        owner: NodeId,
        slot: u32,
        revision: u64,
    },
    WindowCloseRequested {
        target: NodeId,
    },
    WindowSizeChanged {
        target: NodeId,
        size: WindowSize,
    },
    WindowColorSchemeChanged {
        target: NodeId,
        scheme: ColorScheme,
    },
    Click {
        target: NodeId,
    },
    MenuItemClick {
        target: NodeId,
        key: u64,
    },
    TextChanged {
        target: NodeId,
        value: String,
    },
    PasswordChanged {
        target: NodeId,
        value: String,
    },
    Toggled {
        target: NodeId,
        value: bool,
    },
    ValueChanged {
        target: NodeId,
        value: f64,
    },
    OptionalValueChanged {
        target: NodeId,
        value: Option<f64>,
    },
    ColorChanged {
        target: NodeId,
        value: Color,
    },
    DateChanged {
        target: NodeId,
        value: Option<DateTime>,
    },
    TimeChanged {
        target: NodeId,
        value: Option<TimeSpan>,
    },
    DatesChanged {
        target: NodeId,
        value: Vec<DateTime>,
    },
    KeyboardAcceleratorInvoked {
        target: NodeId,
        accelerator: KeyboardAcceleratorSpec,
    },
    Pointer {
        target: NodeId,
        kind: PointerEventKind,
        event: PointerEvent,
    },
    Tapped {
        target: NodeId,
    },
    RightTapped {
        target: NodeId,
    },
    Drop {
        target: NodeId,
        result: Box<windows_core::Result<DropEvent>>,
    },
    Scroll {
        target: NodeId,
        event: ScrollEvent,
    },
    PaneClosed {
        target: NodeId,
    },
    NavigationPaneOpenChanged {
        target: NodeId,
        open: bool,
    },
    NavigationDisplayModeChanged {
        target: NodeId,
        mode: crate::element::NavigationDisplayMode,
    },
    ExpandedChanged {
        target: NodeId,
        expanded: bool,
    },
    TreeNodeExpandedChanged {
        target: NodeId,
        key: u64,
        expanded: bool,
    },
    TeachingTipClosed {
        target: NodeId,
    },
    TeachingTipAction {
        target: NodeId,
    },
    InfoBarCloseRequested {
        target: NodeId,
    },
    TitleBarBackRequested {
        target: NodeId,
    },
    TitleBarPaneRequested {
        target: NodeId,
    },
    FlyoutOpened {
        target: NodeId,
    },
    FlyoutClosed {
        target: NodeId,
    },
    ContentDialogClosed {
        target: NodeId,
        result: ContentDialogResult,
    },
    ImageLoad {
        target: NodeId,
        source_revision: u64,
        result: windows_core::Result<()>,
    },
    CompositionLayout {
        target: NodeId,
        width: f32,
        height: f32,
        rasterization_scale: f32,
    },
    #[cfg(feature = "canvas")]
    CanvasImageLayout {
        target: NodeId,
        width: f32,
        height: f32,
        scale: f32,
    },
    #[cfg(feature = "canvas")]
    CanvasImageFrame {
        target: NodeId,
    },
    #[cfg(feature = "canvas")]
    CanvasLayout {
        target: NodeId,
        width: f32,
        height: f32,
        scale_x: f32,
        scale_y: f32,
    },
    #[cfg(feature = "canvas")]
    CanvasFrame {
        target: NodeId,
    },
    #[cfg(feature = "canvas")]
    SwapChainHostLayout {
        target: NodeId,
        layout: Box<crate::canvas::SwapChainHostLayout>,
    },
    #[cfg(feature = "canvas")]
    SwapChainHostFrame {
        target: NodeId,
    },
    #[cfg(feature = "webview")]
    WebViewInitializationReady {
        target: NodeId,
        revision: u64,
    },
    #[cfg(feature = "webview")]
    WebViewCreated {
        target: NodeId,
        result: windows_core::Result<()>,
    },
    #[cfg(feature = "webview")]
    WebViewNavigationCompleted {
        target: NodeId,
        navigation_id: u64,
        is_success: bool,
        source: Box<str>,
    },
    DeferredReady {
        target: NodeId,
        revision: u64,
        action: DeferredAction,
    },
    ItemInvoked {
        target: NodeId,
        key: u64,
    },
    QuerySubmitted {
        target: NodeId,
        value: String,
    },
    SelectionChanged {
        target: NodeId,
        selection: CollectionSelection,
    },
    IndexChanged {
        target: NodeId,
        index: Option<usize>,
    },
    TabCloseRequested {
        target: NodeId,
        key: u64,
    },
    AddTabButtonClick {
        target: NodeId,
    },
    TabsReordered {
        target: NodeId,
        keys: Vec<u64>,
    },
    ItemsReordered {
        target: NodeId,
        keys: Vec<u64>,
    },
    SelectedKeyChanged {
        target: NodeId,
        key: Option<u64>,
    },
    Realize {
        host: NodeId,
        index: usize,
        lease: u64,
    },
    Recycle {
        host: NodeId,
        index: usize,
        lease: u64,
    },
}

impl NativeEvent {
    pub(crate) const fn class(&self) -> NativeEventClass {
        match self {
            Self::TextChanged { .. }
            | Self::PasswordChanged { .. }
            | Self::Toggled { .. }
            | Self::ValueChanged { .. }
            | Self::OptionalValueChanged { .. }
            | Self::ColorChanged { .. }
            | Self::DateChanged { .. }
            | Self::TimeChanged { .. }
            | Self::DatesChanged { .. }
            | Self::PaneClosed { .. }
            | Self::NavigationPaneOpenChanged { .. }
            | Self::ExpandedChanged { .. }
            | Self::TreeNodeExpandedChanged { .. }
            | Self::SelectionChanged { .. }
            | Self::IndexChanged { .. }
            | Self::TabsReordered { .. }
            | Self::ItemsReordered { .. }
            | Self::SelectedKeyChanged { .. } => NativeEventClass::ControlledFeedback,
            Self::WindowSizeChanged { .. }
            | Self::WindowColorSchemeChanged { .. }
            | Self::Click { .. }
            | Self::MenuItemClick { .. }
            | Self::KeyboardAcceleratorInvoked { .. }
            | Self::Pointer { .. }
            | Self::Tapped { .. }
            | Self::RightTapped { .. }
            | Self::Drop { .. }
            | Self::Scroll { .. }
            | Self::NavigationDisplayModeChanged { .. }
            | Self::TeachingTipAction { .. }
            | Self::TitleBarBackRequested { .. }
            | Self::TitleBarPaneRequested { .. }
            | Self::FlyoutOpened { .. }
            | Self::FlyoutClosed { .. }
            | Self::ImageLoad { .. }
            | Self::ItemInvoked { .. }
            | Self::QuerySubmitted { .. }
            | Self::TabCloseRequested { .. }
            | Self::AddTabButtonClick { .. } => NativeEventClass::NotificationOnly,
            #[cfg(feature = "webview")]
            Self::WebViewCreated { .. } | Self::WebViewNavigationCompleted { .. } => {
                NativeEventClass::NotificationOnly
            }
            Self::WindowCloseRequested { .. } | Self::InfoBarCloseRequested { .. } => {
                NativeEventClass::CancelableRequest
            }
            Self::TeachingTipClosed { .. } | Self::ContentDialogClosed { .. } => {
                NativeEventClass::ClosureFailureSynchronization
            }
            Self::TimerFired { .. }
            | Self::DeferredReady { .. }
            | Self::CompositionLayout { .. }
            | Self::Realize { .. }
            | Self::Recycle { .. } => NativeEventClass::InternalRuntime,
            #[cfg(feature = "webview")]
            Self::WebViewInitializationReady { .. } => NativeEventClass::InternalRuntime,
            #[cfg(feature = "canvas")]
            Self::CanvasImageLayout { .. }
            | Self::CanvasImageFrame { .. }
            | Self::CanvasLayout { .. }
            | Self::CanvasFrame { .. }
            | Self::SwapChainHostLayout { .. }
            | Self::SwapChainHostFrame { .. } => NativeEventClass::InternalRuntime,
        }
    }

    pub(crate) const fn compatibility(&self) -> NativeEventCompatibility {
        match self {
            Self::TimerFired { .. } => NativeEventCompatibility::TimerOwner,
            Self::WindowCloseRequested { .. }
            | Self::WindowSizeChanged { .. }
            | Self::WindowColorSchemeChanged { .. } => NativeEventCompatibility::Window,
            Self::Click { .. } => NativeEventCompatibility::Click,
            Self::MenuItemClick { .. } => NativeEventCompatibility::Menu,
            Self::TextChanged { .. } => NativeEventCompatibility::Text,
            Self::PasswordChanged { .. } => NativeEventCompatibility::Password,
            Self::Toggled { .. } => NativeEventCompatibility::Toggle,
            Self::ValueChanged { .. } => NativeEventCompatibility::Slider,
            Self::OptionalValueChanged { .. } => NativeEventCompatibility::OptionalValue,
            Self::ColorChanged { .. } => NativeEventCompatibility::Color,
            Self::DateChanged { .. } => NativeEventCompatibility::Date,
            Self::TimeChanged { .. } => NativeEventCompatibility::Time,
            Self::DatesChanged { .. } => NativeEventCompatibility::Dates,
            Self::KeyboardAcceleratorInvoked { .. }
            | Self::Pointer { .. }
            | Self::Tapped { .. }
            | Self::RightTapped { .. }
            | Self::Drop { .. } => NativeEventCompatibility::Framework,
            Self::Scroll { .. } => NativeEventCompatibility::Scroll,
            Self::PaneClosed { .. } => NativeEventCompatibility::SplitView,
            Self::NavigationPaneOpenChanged { .. } | Self::NavigationDisplayModeChanged { .. } => {
                NativeEventCompatibility::NavigationView
            }
            Self::ExpandedChanged { .. } => NativeEventCompatibility::Expander,
            Self::TreeNodeExpandedChanged { .. } => NativeEventCompatibility::TreeView,
            Self::TeachingTipClosed { .. } | Self::TeachingTipAction { .. } => {
                NativeEventCompatibility::TeachingTip
            }
            Self::InfoBarCloseRequested { .. } => NativeEventCompatibility::InfoBar,
            Self::TitleBarBackRequested { .. } | Self::TitleBarPaneRequested { .. } => {
                NativeEventCompatibility::TitleBar
            }
            Self::FlyoutOpened { .. } | Self::FlyoutClosed { .. } => {
                NativeEventCompatibility::Flyout
            }
            Self::ContentDialogClosed { .. } => NativeEventCompatibility::ContentDialog,
            Self::ImageLoad { .. } => NativeEventCompatibility::Image,
            Self::CompositionLayout { .. } => NativeEventCompatibility::CompositionHost,
            #[cfg(feature = "webview")]
            Self::WebViewInitializationReady { .. }
            | Self::WebViewCreated { .. }
            | Self::WebViewNavigationCompleted { .. } => NativeEventCompatibility::WebViewHost,
            #[cfg(feature = "canvas")]
            Self::CanvasImageLayout { .. } | Self::CanvasImageFrame { .. } => {
                NativeEventCompatibility::CanvasImage
            }
            #[cfg(feature = "canvas")]
            Self::CanvasLayout { .. } | Self::CanvasFrame { .. } => {
                NativeEventCompatibility::SwapChainCanvas
            }
            #[cfg(feature = "canvas")]
            Self::SwapChainHostLayout { .. } | Self::SwapChainHostFrame { .. } => {
                NativeEventCompatibility::SwapChainHost
            }
            Self::DeferredReady { action, .. } => match action {
                DeferredAction::ContentDialogOpen => {
                    NativeEventCompatibility::DeferredContentDialog
                }
                DeferredAction::TeachingTipOpen => NativeEventCompatibility::DeferredTeachingTip,
                DeferredAction::RadioButtonsSelection => {
                    NativeEventCompatibility::DeferredRadioButtons
                }
            },
            Self::ItemInvoked { .. } => NativeEventCompatibility::ItemInvocation,
            Self::QuerySubmitted { .. } => NativeEventCompatibility::AutoSuggestBox,
            Self::SelectionChanged { .. } => NativeEventCompatibility::CollectionSelection,
            Self::IndexChanged { .. } => NativeEventCompatibility::IndexSelection,
            Self::TabCloseRequested { .. }
            | Self::AddTabButtonClick { .. }
            | Self::TabsReordered { .. } => NativeEventCompatibility::TabView,
            Self::ItemsReordered { .. } => NativeEventCompatibility::VirtualCollection,
            Self::SelectedKeyChanged { .. } => NativeEventCompatibility::SingleSelection,
            Self::Realize { .. } | Self::Recycle { .. } => NativeEventCompatibility::VirtualHost,
        }
    }

    pub(crate) fn target(&self) -> NodeId {
        match self {
            Self::TimerFired { owner, .. } => *owner,
            Self::Realize { host, .. } | Self::Recycle { host, .. } => *host,
            Self::WindowCloseRequested { target }
            | Self::WindowSizeChanged { target, .. }
            | Self::WindowColorSchemeChanged { target, .. }
            | Self::Click { target }
            | Self::MenuItemClick { target, .. }
            | Self::TextChanged { target, .. }
            | Self::PasswordChanged { target, .. }
            | Self::Toggled { target, .. }
            | Self::ValueChanged { target, .. }
            | Self::OptionalValueChanged { target, .. }
            | Self::ColorChanged { target, .. }
            | Self::DateChanged { target, .. }
            | Self::TimeChanged { target, .. }
            | Self::DatesChanged { target, .. }
            | Self::KeyboardAcceleratorInvoked { target, .. }
            | Self::Pointer { target, .. }
            | Self::Tapped { target }
            | Self::RightTapped { target }
            | Self::Drop { target, .. }
            | Self::Scroll { target, .. }
            | Self::PaneClosed { target }
            | Self::NavigationPaneOpenChanged { target, .. }
            | Self::NavigationDisplayModeChanged { target, .. }
            | Self::ExpandedChanged { target, .. }
            | Self::TreeNodeExpandedChanged { target, .. }
            | Self::TeachingTipClosed { target }
            | Self::TeachingTipAction { target }
            | Self::InfoBarCloseRequested { target }
            | Self::TitleBarBackRequested { target }
            | Self::TitleBarPaneRequested { target }
            | Self::FlyoutOpened { target }
            | Self::FlyoutClosed { target }
            | Self::ContentDialogClosed { target, .. }
            | Self::ImageLoad { target, .. }
            | Self::CompositionLayout { target, .. }
            | Self::DeferredReady { target, .. }
            | Self::ItemInvoked { target, .. }
            | Self::QuerySubmitted { target, .. }
            | Self::SelectionChanged { target, .. } => *target,
            #[cfg(feature = "webview")]
            Self::WebViewInitializationReady { target, .. }
            | Self::WebViewCreated { target, .. }
            | Self::WebViewNavigationCompleted { target, .. } => *target,
            Self::IndexChanged { target, .. }
            | Self::TabCloseRequested { target, .. }
            | Self::AddTabButtonClick { target }
            | Self::TabsReordered { target, .. }
            | Self::ItemsReordered { target, .. } => *target,
            Self::SelectedKeyChanged { target, .. } => *target,
            #[cfg(feature = "canvas")]
            Self::CanvasImageLayout { target, .. }
            | Self::CanvasImageFrame { target }
            | Self::CanvasLayout { target, .. }
            | Self::CanvasFrame { target }
            | Self::SwapChainHostLayout { target, .. }
            | Self::SwapChainHostFrame { target } => *target,
        }
    }

    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::TimerFired { .. } => "TimerFired",
            Self::WindowCloseRequested { .. } => "WindowCloseRequested",
            Self::WindowSizeChanged { .. } => "WindowSizeChanged",
            Self::WindowColorSchemeChanged { .. } => "WindowColorSchemeChanged",
            Self::Click { .. } => "Click",
            Self::MenuItemClick { .. } => "MenuItemClick",
            Self::TextChanged { .. } => "TextChanged",
            Self::PasswordChanged { .. } => "PasswordChanged",
            Self::Toggled { .. } => "Toggled",
            Self::ValueChanged { .. } => "ValueChanged",
            Self::OptionalValueChanged { .. } => "OptionalValueChanged",
            Self::ColorChanged { .. } => "ColorChanged",
            Self::DateChanged { .. } => "DateChanged",
            Self::TimeChanged { .. } => "TimeChanged",
            Self::DatesChanged { .. } => "DatesChanged",
            Self::KeyboardAcceleratorInvoked { .. } => "KeyboardAcceleratorInvoked",
            Self::Pointer { .. } => "Pointer",
            Self::Tapped { .. } => "Tapped",
            Self::RightTapped { .. } => "RightTapped",
            Self::Drop { .. } => "Drop",
            Self::Scroll { .. } => "Scroll",
            Self::PaneClosed { .. } => "PaneClosed",
            Self::NavigationPaneOpenChanged { .. } => "NavigationPaneOpenChanged",
            Self::NavigationDisplayModeChanged { .. } => "NavigationDisplayModeChanged",
            Self::ExpandedChanged { .. } => "ExpandedChanged",
            Self::TreeNodeExpandedChanged { .. } => "TreeNodeExpandedChanged",
            Self::TeachingTipClosed { .. } => "TeachingTipClosed",
            Self::TeachingTipAction { .. } => "TeachingTipAction",
            Self::InfoBarCloseRequested { .. } => "InfoBarCloseRequested",
            Self::TitleBarBackRequested { .. } => "TitleBarBackRequested",
            Self::TitleBarPaneRequested { .. } => "TitleBarPaneRequested",
            Self::FlyoutOpened { .. } => "FlyoutOpened",
            Self::FlyoutClosed { .. } => "FlyoutClosed",
            Self::ContentDialogClosed { .. } => "ContentDialogClosed",
            Self::ImageLoad { .. } => "ImageLoad",
            Self::CompositionLayout { .. } => "CompositionLayout",
            #[cfg(feature = "webview")]
            Self::WebViewInitializationReady { .. } => "WebViewInitializationReady",
            #[cfg(feature = "webview")]
            Self::WebViewCreated { .. } => "WebViewCreated",
            #[cfg(feature = "webview")]
            Self::WebViewNavigationCompleted { .. } => "WebViewNavigationCompleted",
            #[cfg(feature = "canvas")]
            Self::CanvasImageLayout { .. } => "CanvasImageLayout",
            #[cfg(feature = "canvas")]
            Self::CanvasImageFrame { .. } => "CanvasImageFrame",
            #[cfg(feature = "canvas")]
            Self::CanvasLayout { .. } => "CanvasLayout",
            #[cfg(feature = "canvas")]
            Self::CanvasFrame { .. } => "CanvasFrame",
            #[cfg(feature = "canvas")]
            Self::SwapChainHostLayout { .. } => "SwapChainHostLayout",
            #[cfg(feature = "canvas")]
            Self::SwapChainHostFrame { .. } => "SwapChainHostFrame",
            Self::DeferredReady { .. } => "DeferredReady",
            Self::ItemInvoked { .. } => "ItemInvoked",
            Self::QuerySubmitted { .. } => "QuerySubmitted",
            Self::SelectionChanged { .. } => "SelectionChanged",
            Self::IndexChanged { .. } => "IndexChanged",
            Self::TabCloseRequested { .. } => "TabCloseRequested",
            Self::AddTabButtonClick { .. } => "AddTabButtonClick",
            Self::TabsReordered { .. } => "TabsReordered",
            Self::ItemsReordered { .. } => "ItemsReordered",
            Self::SelectedKeyChanged { .. } => "SelectedKeyChanged",
            Self::Realize { .. } => "Realize",
            Self::Recycle { .. } => "Recycle",
        }
    }
}

pub trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]);
    fn drain_events(&mut self) -> Vec<NativeEvent>;
    fn set_event_waker(&mut self, _waker: Option<Rc<dyn Fn()>>) {}
}
