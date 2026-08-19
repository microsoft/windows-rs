use std::any::TypeId;

use crate::element::props::*;
use crate::element::*;
use crate::framework_properties::FrameworkProps;
use crate::hooks::{ComponentMemo, RenderFn};
use crate::references::NativeElementRef;
use crate::resources::ContextProps;
use crate::runtime::NativeKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReconcileKind {
    Application,
    Window,
    Component,
    Fragment,
    StructuralSlot(StructuralSlot),
    Context,
    Reference,
    FadeTransition,
    ButtonFlyout,
    ButtonMenuFlyout,
    ButtonCommandBarFlyout,
    DropDownMenuFlyout,
    SplitButtonFlyout,
    AttachedChild,
    VirtualCollection,
    CommandSection,
    NavigationSection,
    TeachingTipOwner,
    ToolTipOwner,
    Native(NativeKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StructuralSlot {
    Content,
    Pane,
    Header,
}

pub struct ToolTipElement {
    pub owner: Box<Element>,
    pub content: Box<Element>,
    pub placement: Option<TooltipPlacement>,
}

pub struct TeachingTipElement {
    pub owner: Box<Element>,
    pub props: TeachingTipProps,
}

pub struct SplitViewElement {
    pub content: Box<Element>,
    pub pane: Box<Element>,
    pub props: SplitViewProps,
}

pub struct ExpanderElement {
    pub header: Box<Element>,
    pub content: Box<Element>,
    pub props: ExpanderProps,
}

pub struct ContentDialogElement {
    pub title: Box<Element>,
    pub content: Box<Element>,
    pub props: ContentDialogProps,
}

pub struct BorderElement {
    pub child: Box<Element>,
    pub props: BorderProps,
}

pub struct TitleBarElement {
    pub content: Box<Element>,
    pub right_header: Box<Element>,
    pub props: TitleBarProps,
}

pub struct WindowElement {
    pub title_bar: Box<Element>,
    pub content: Box<Element>,
    pub owned_windows: Vec<Element>,
    pub props: WindowProps,
    pub custom_title_bar: bool,
}

pub struct DropDownButtonElement {
    pub label: Box<Element>,
    pub flyout: DropDownFlyoutElement,
    pub props: DropDownButtonProps,
}

pub enum DropDownFlyoutElement {
    Content(Box<Element>),
    Menu(MenuFlyoutProps),
}

pub struct ButtonFlyoutElement {
    pub label: Box<Element>,
    pub flyout: Box<Element>,
    pub flyout_props: FlyoutProps,
}

pub struct NavigationViewElement {
    pub items: Vec<Element>,
    pub content: Box<Element>,
    pub footer: Option<Box<Element>>,
    pub props: NavigationViewProps,
}

pub enum ElementKind {
    Application {
        windows: Vec<Element>,
        props: ApplicationProps,
    },
    Window(Box<WindowElement>),
    Component {
        identity: TypeId,
        render: RenderFn,
        memo: Option<ComponentMemo>,
    },
    Fragment {
        children: Vec<Element>,
    },
    StructuralSlot {
        slot: StructuralSlot,
        child: Box<Element>,
    },
    Context {
        props: ContextProps,
        child: Box<Element>,
    },
    Reference {
        reference: NativeElementRef,
        child: Box<Element>,
    },
    FadeTransition {
        child: Box<Element>,
        enter: Option<std::time::Duration>,
        exit: Option<std::time::Duration>,
    },
    StackPanel(StackPanelProps),
    Grid(GridProps),
    TitleBar(Box<TitleBarElement>),
    Canvas(PanelProps),
    RelativePanel(PanelProps),
    Viewbox {
        child: Box<Element>,
        props: ViewboxProps,
    },
    ScrollViewer {
        child: Box<Element>,
        props: ScrollViewerProps,
    },
    ScrollView {
        child: Box<Element>,
        props: ScrollViewProps,
    },
    SplitView(Box<SplitViewElement>),
    Expander(Box<ExpanderElement>),
    CommandBar(Box<CommandBarProps>),
    CompositionHost(Box<crate::composition::CompositionHostProps>),
    #[cfg(feature = "canvas")]
    CanvasImage(crate::canvas::CanvasImageProps),
    #[cfg(feature = "canvas")]
    SwapChainCanvas(crate::canvas::SwapChainCanvasProps),
    #[cfg(feature = "canvas")]
    SwapChainHost(Box<crate::canvas::SwapChainHostProps>),
    #[cfg(feature = "webview")]
    WebViewHost(Box<crate::webview::WebViewHostProps>),
    Image(ImageProps),
    Icon(Icon),
    NavigationView(Box<NavigationViewElement>),
    NavigationViewItem(NavigationViewItemProps),
    Shape(Box<ShapeProps>),
    AppBarButton(AppBarButtonProps),
    AppBarToggleButton(AppBarToggleButtonProps),
    AppBarSeparator,
    ContentDialog(Box<ContentDialogElement>),
    TeachingTip(Box<TeachingTipElement>),
    ToolTip(Box<ToolTipElement>),
    AttachedChild {
        placement: AttachedPlacement,
        child: Box<Element>,
    },
    Border(Box<BorderElement>),
    Button {
        child: Box<Element>,
        props: ButtonProps,
    },
    ButtonFlyout {
        button: ButtonProps,
        content: Box<ButtonFlyoutElement>,
    },
    ButtonMenuFlyout {
        button: ButtonProps,
        label: Box<Element>,
        flyout: MenuFlyoutProps,
    },
    ButtonCommandBarFlyout {
        button: ButtonProps,
        label: Box<Element>,
        flyout: Box<CommandBarFlyoutProps>,
    },
    DropDownButton(Box<DropDownButtonElement>),
    SplitButton {
        child: Box<Element>,
        props: SplitButtonProps,
    },
    SplitButtonFlyout {
        button: SplitButtonProps,
        content: Box<ButtonFlyoutElement>,
    },
    HyperlinkButton {
        child: Box<Element>,
        props: HyperlinkButtonProps,
    },
    RepeatButton {
        child: Box<Element>,
        props: RepeatButtonProps,
    },
    ToggleButton {
        child: Box<Element>,
        props: ToggleButtonProps,
    },
    ToggleSwitch(ToggleSwitchProps),
    InfoBadge(InfoBadgeProps),
    InfoBar(Box<InfoBarProps>),
    PersonPicture(Box<PersonPictureProps>),
    ProgressBar(ProgressBarProps),
    ProgressRing(ProgressRingProps),
    Slider(SliderProps),
    NumberBox(NumberBoxProps),
    RatingControl(RatingControlProps),
    ColorPicker(ColorPickerProps),
    DatePicker(DatePickerProps),
    CalendarDatePicker(CalendarDatePickerProps),
    TimePicker(TimePickerProps),
    CalendarView(CalendarViewProps),
    RichEditBox(Box<RichEditBoxProps>),
    RichTextBlock(Box<RichTextBlockProps>),
    TreeView(TreeViewProps),
    CheckBox {
        child: Box<Element>,
        props: CheckBoxProps,
    },
    RadioButton {
        child: Box<Element>,
        props: RadioButtonProps,
    },
    TextBlock(TextBlockProps),
    TextBox(Box<TextBoxProps>),
    PasswordBox(PasswordBoxProps),
    ListBox(ListBoxProps),
    ComboBox(ComboBoxProps),
    RadioButtons(RadioButtonsProps),
    MenuBar(MenuBarProps),
    FlipView(Box<FlipViewProps>),
    TabView(Box<TabViewProps>),
    TabViewItem {
        child: Box<Element>,
        props: TabViewItemProps,
    },
    SelectorBar(Box<SelectorBarProps>),
    SelectorBarItem(SelectorBarItemProps),
    BreadcrumbBar(BreadcrumbBarProps),
    AutoSuggestBox(AutoSuggestBoxProps),
    Pivot(Box<PivotProps>),
    PivotItem {
        child: Box<Element>,
        props: PivotItemProps,
    },
    VirtualCollection(Box<VirtualCollectionProps>),
}

impl ElementKind {
    pub(crate) fn reconcile_kind(&self) -> ReconcileKind {
        match self {
            Self::Application { .. } => ReconcileKind::Application,
            Self::Window(_) => ReconcileKind::Window,
            Self::Component { .. } => ReconcileKind::Component,
            Self::Fragment { .. } => ReconcileKind::Fragment,
            Self::StructuralSlot { slot, .. } => ReconcileKind::StructuralSlot(*slot),
            Self::Context { .. } => ReconcileKind::Context,
            Self::Reference { .. } => ReconcileKind::Reference,
            Self::FadeTransition { .. } => ReconcileKind::FadeTransition,
            Self::StackPanel(_) => ReconcileKind::Native(NativeKind::StackPanel),
            Self::Grid(_) => ReconcileKind::Native(NativeKind::Grid),
            Self::TitleBar(_) => ReconcileKind::Native(NativeKind::TitleBar),
            Self::Canvas(_) => ReconcileKind::Native(NativeKind::Canvas),
            Self::RelativePanel(_) => ReconcileKind::Native(NativeKind::RelativePanel),
            Self::Viewbox { .. } => ReconcileKind::Native(NativeKind::Viewbox),
            Self::ScrollViewer { .. } => ReconcileKind::Native(NativeKind::ScrollViewer),
            Self::ScrollView { .. } => ReconcileKind::Native(NativeKind::ScrollView),
            Self::SplitView(_) => ReconcileKind::Native(NativeKind::SplitView),
            Self::Expander(_) => ReconcileKind::Native(NativeKind::Expander),
            Self::CommandBar(_) => ReconcileKind::Native(NativeKind::CommandBar),
            Self::CompositionHost(_) => ReconcileKind::Native(NativeKind::CompositionHost),
            #[cfg(feature = "canvas")]
            Self::CanvasImage(_) => ReconcileKind::Native(NativeKind::CanvasImage),
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(_) => ReconcileKind::Native(NativeKind::SwapChainCanvas),
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(_) => ReconcileKind::Native(NativeKind::SwapChainHost),
            #[cfg(feature = "webview")]
            Self::WebViewHost(_) => ReconcileKind::Native(NativeKind::WebViewHost),
            Self::Image(_) => ReconcileKind::Native(NativeKind::Image),
            Self::Icon(icon) => ReconcileKind::Native(match icon.kind() {
                IconKind::Symbol(_) => NativeKind::SymbolIcon,
                IconKind::Font { .. } => NativeKind::FontIcon,
                IconKind::Bitmap { .. } => NativeKind::BitmapIcon,
                IconKind::Image(_) => NativeKind::ImageIcon,
                IconKind::Path(_) => NativeKind::PathIcon,
            }),
            Self::NavigationView(_) => ReconcileKind::Native(NativeKind::NavigationView),
            Self::NavigationViewItem(_) => ReconcileKind::Native(NativeKind::NavigationViewItem),
            Self::Shape(props) => ReconcileKind::Native(props.kind.native_kind()),
            Self::AppBarButton(_) => ReconcileKind::Native(NativeKind::AppBarButton),
            Self::AppBarToggleButton(_) => ReconcileKind::Native(NativeKind::AppBarToggleButton),
            Self::AppBarSeparator => ReconcileKind::Native(NativeKind::AppBarSeparator),
            Self::ContentDialog(_) => ReconcileKind::Native(NativeKind::ContentDialog),
            Self::TeachingTip(_) => ReconcileKind::TeachingTipOwner,
            Self::ToolTip(_) => ReconcileKind::ToolTipOwner,
            Self::AttachedChild { .. } => ReconcileKind::AttachedChild,
            Self::Border(_) => ReconcileKind::Native(NativeKind::Border),
            Self::Button { .. } => ReconcileKind::Native(NativeKind::Button),
            Self::ButtonFlyout { .. } => ReconcileKind::ButtonFlyout,
            Self::ButtonMenuFlyout { .. } => ReconcileKind::ButtonMenuFlyout,
            Self::ButtonCommandBarFlyout { .. } => ReconcileKind::ButtonCommandBarFlyout,
            Self::DropDownButton(drop_down) => match drop_down.flyout {
                DropDownFlyoutElement::Content(_) => {
                    ReconcileKind::Native(NativeKind::DropDownButton)
                }
                DropDownFlyoutElement::Menu(_) => ReconcileKind::DropDownMenuFlyout,
            },
            Self::SplitButton { .. } => ReconcileKind::Native(NativeKind::SplitButton),
            Self::SplitButtonFlyout { .. } => ReconcileKind::SplitButtonFlyout,
            Self::HyperlinkButton { .. } => ReconcileKind::Native(NativeKind::HyperlinkButton),
            Self::RepeatButton { .. } => ReconcileKind::Native(NativeKind::RepeatButton),
            Self::ToggleButton { .. } => ReconcileKind::Native(NativeKind::ToggleButton),
            Self::ToggleSwitch(_) => ReconcileKind::Native(NativeKind::ToggleSwitch),
            Self::InfoBadge(_) => ReconcileKind::Native(NativeKind::InfoBadge),
            Self::InfoBar(_) => ReconcileKind::Native(NativeKind::InfoBar),
            Self::PersonPicture(_) => ReconcileKind::Native(NativeKind::PersonPicture),
            Self::ProgressBar(_) => ReconcileKind::Native(NativeKind::ProgressBar),
            Self::ProgressRing(_) => ReconcileKind::Native(NativeKind::ProgressRing),
            Self::Slider(_) => ReconcileKind::Native(NativeKind::Slider),
            Self::NumberBox(_) => ReconcileKind::Native(NativeKind::NumberBox),
            Self::RatingControl(_) => ReconcileKind::Native(NativeKind::RatingControl),
            Self::ColorPicker(_) => ReconcileKind::Native(NativeKind::ColorPicker),
            Self::DatePicker(_) => ReconcileKind::Native(NativeKind::DatePicker),
            Self::CalendarDatePicker(_) => ReconcileKind::Native(NativeKind::CalendarDatePicker),
            Self::TimePicker(_) => ReconcileKind::Native(NativeKind::TimePicker),
            Self::CalendarView(_) => ReconcileKind::Native(NativeKind::CalendarView),
            Self::RichEditBox(_) => ReconcileKind::Native(NativeKind::RichEditBox),
            Self::RichTextBlock(_) => ReconcileKind::Native(NativeKind::RichTextBlock),
            Self::TreeView(_) => ReconcileKind::Native(NativeKind::TreeView),
            Self::CheckBox { .. } => ReconcileKind::Native(NativeKind::CheckBox),
            Self::RadioButton { .. } => ReconcileKind::Native(NativeKind::RadioButton),
            Self::TextBlock(_) => ReconcileKind::Native(NativeKind::TextBlock),
            Self::TextBox(_) => ReconcileKind::Native(NativeKind::TextBox),
            Self::PasswordBox(_) => ReconcileKind::Native(NativeKind::PasswordBox),
            Self::ListBox(_) => ReconcileKind::Native(NativeKind::ListBox),
            Self::ComboBox(_) => ReconcileKind::Native(NativeKind::ComboBox),
            Self::RadioButtons(_) => ReconcileKind::Native(NativeKind::RadioButtons),
            Self::MenuBar(_) => ReconcileKind::Native(NativeKind::MenuBar),
            Self::FlipView(_) => ReconcileKind::Native(NativeKind::FlipView),
            Self::TabView(_) => ReconcileKind::Native(NativeKind::TabView),
            Self::TabViewItem { .. } => ReconcileKind::Native(NativeKind::TabViewItem),
            Self::SelectorBar(_) => ReconcileKind::Native(NativeKind::SelectorBar),
            Self::SelectorBarItem(_) => ReconcileKind::Native(NativeKind::SelectorBarItem),
            Self::BreadcrumbBar(_) => ReconcileKind::Native(NativeKind::BreadcrumbBar),
            Self::AutoSuggestBox(_) => ReconcileKind::Native(NativeKind::AutoSuggestBox),
            Self::Pivot(_) => ReconcileKind::Native(NativeKind::Pivot),
            Self::PivotItem { .. } => ReconcileKind::Native(NativeKind::PivotItem),
            Self::VirtualCollection(_) => ReconcileKind::VirtualCollection,
        }
    }
}

macro_rules! define_element_framework_prop_lookups {
    ($($(#[$attr:meta])* ($control:ident, $element_pattern:pat => $element_props:expr, $mounted_pattern:pat => $mounted_props:expr),)*) => {
        impl ElementKind {
            pub fn framework_props(&self) -> Option<&FrameworkProps> {
                match self {
                    $($(#[$attr])* $element_pattern => Some($element_props),)*
                    _ => None,
                }
            }
        }
    };
}

framework_elements!(define_element_framework_prop_lookups);
