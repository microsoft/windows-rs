use std::any::TypeId;
use std::rc::Rc;

use crate::element::props::*;
use crate::element::tree::{ReconcileKind, StructuralSlot};
use crate::element::*;
use crate::framework_properties::FrameworkProps;
use crate::hooks::{Cleanup, ComponentMemo, HookCell, RenderFn};
use crate::id::NodeId;
use crate::references::NativeElementRef;
use crate::resources::{ContextEntry, ContextProps};
use crate::runtime::NativeKind;

pub(crate) enum MountedKind {
    Application(ApplicationProps),
    Window(MountedWindow),
    Component {
        identity: TypeId,
        render: RenderFn,
        memo: Option<ComponentMemo>,
        contexts: Vec<ContextEntry>,
        hooks: Vec<Rc<HookCell>>,
    },
    Fragment,
    StructuralSlot(StructuralSlot),
    Context(ContextProps),
    Reference {
        reference: NativeElementRef,
        target: NodeId,
    },
    FadeTransition {
        enter: Option<std::time::Duration>,
        exit: Option<std::time::Duration>,
        revision: u64,
        exiting: bool,
    },
    StackPanel(StackPanelState),
    Grid(GridState),
    TitleBar(Box<TitleBarProps>),
    Canvas(FrameworkProps),
    RelativePanel(FrameworkProps),
    Viewbox(ViewboxProps),
    ScrollViewer(ScrollViewerProps),
    ScrollView(ScrollViewProps),
    SplitView(Box<SplitViewProps>),
    Expander(Box<ExpanderProps>),
    CommandBar {
        default_label_position: CommandBarDefaultLabelPosition,
        framework: FrameworkProps,
    },
    CompositionHost(Box<crate::composition::CompositionHostProps>),
    #[cfg(feature = "canvas")]
    CanvasImage(crate::canvas::CanvasImageProps),
    #[cfg(feature = "canvas")]
    SwapChainCanvas(crate::canvas::SwapChainCanvasProps),
    #[cfg(feature = "canvas")]
    SwapChainHost(Box<crate::canvas::SwapChainHostProps>),
    #[cfg(feature = "webview")]
    WebViewHost(Box<crate::webview::WebViewHostProps>),
    Image {
        props: ImageProps,
        source_revision: u64,
    },
    Icon(Icon),
    NavigationView(Box<NavigationViewProps>),
    NavigationViewItem(NavigationViewItemProps),
    NavigationSection,
    Shape(Box<ShapeProps>),
    CommandSection,
    AppBarButton(AppBarButtonProps),
    AppBarToggleButton(AppBarToggleButtonProps),
    AppBarSeparator,
    ContentDialog(ContentDialogProps),
    TeachingTip(TeachingTipProps),
    TeachingTipOwner,
    ToolTip(Option<TooltipPlacement>),
    AttachedChild(AttachedPlacement),
    Border(BorderProps),
    Button(ButtonProps),
    ButtonEvent(Option<EventFn>),
    ButtonFlyout(ButtonProps),
    ButtonMenuFlyout(ButtonProps),
    ButtonCommandBarFlyout(ButtonProps),
    DropDownButton(DropDownButtonProps),
    DropDownMenuFlyout(DropDownButtonProps),
    SplitButton(SplitButtonProps),
    SplitButtonEvent(Option<EventFn>),
    SplitButtonFlyout(SplitButtonProps),
    Flyout(FlyoutProps),
    MenuFlyout(MenuFlyoutProps),
    MenuBar(MenuBarProps),
    CommandBarFlyout(FlyoutProps),
    HyperlinkButton(HyperlinkButtonProps),
    RepeatButton(RepeatButtonProps),
    ToggleButton(ToggleButtonProps),
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
    CheckBox(CheckBoxProps),
    RadioButton(RadioButtonProps),
    TextBlock(TextBlockProps),
    TextBox(Box<TextBoxProps>),
    PasswordBox(PasswordBoxProps),
    ListBox(ListBoxProps),
    ComboBox(ComboBoxProps),
    RadioButtons(RadioButtonsProps),
    FlipView(FlipViewProps),
    TabView(TabViewProps),
    TabViewItem(TabViewItemProps),
    SelectorBar(SelectorBarProps),
    SelectorBarItem(SelectorBarItemProps),
    BreadcrumbBar(BreadcrumbBarProps),
    AutoSuggestBox(AutoSuggestBoxProps),
    Pivot(PivotProps),
    PivotItem(PivotItemProps),
    VirtualCollection(Box<VirtualCollectionProps>),
}

pub(crate) struct MountedWindow {
    pub(crate) props: WindowProps,
    pub(crate) content: NodeId,
    pub(crate) title_bar: Option<NodeId>,
}

pub(crate) struct Mounted {
    pub key: Option<u64>,
    pub kind: MountedKind,
}

impl MountedKind {
    pub(crate) fn reconcile_kind(&self) -> ReconcileKind {
        match self {
            Self::Application(_) => ReconcileKind::Application,
            Self::Window(_) => ReconcileKind::Window,
            Self::Component { .. } => ReconcileKind::Component,
            Self::Fragment => ReconcileKind::Fragment,
            Self::StructuralSlot(slot) => ReconcileKind::StructuralSlot(*slot),
            Self::Context(_) => ReconcileKind::Context,
            Self::Reference { .. } => ReconcileKind::Reference,
            Self::FadeTransition { .. } => ReconcileKind::FadeTransition,
            Self::StackPanel(_) => ReconcileKind::Native(NativeKind::StackPanel),
            Self::Grid(_) => ReconcileKind::Native(NativeKind::Grid),
            Self::TitleBar(_) => ReconcileKind::Native(NativeKind::TitleBar),
            Self::Canvas(_) => ReconcileKind::Native(NativeKind::Canvas),
            Self::RelativePanel(_) => ReconcileKind::Native(NativeKind::RelativePanel),
            Self::Viewbox(_) => ReconcileKind::Native(NativeKind::Viewbox),
            Self::ScrollViewer(_) => ReconcileKind::Native(NativeKind::ScrollViewer),
            Self::ScrollView(_) => ReconcileKind::Native(NativeKind::ScrollView),
            Self::SplitView(_) => ReconcileKind::Native(NativeKind::SplitView),
            Self::Expander(_) => ReconcileKind::Native(NativeKind::Expander),
            Self::CommandBar { .. } => ReconcileKind::Native(NativeKind::CommandBar),
            Self::CompositionHost(_) => ReconcileKind::Native(NativeKind::CompositionHost),
            #[cfg(feature = "canvas")]
            Self::CanvasImage(_) => ReconcileKind::Native(NativeKind::CanvasImage),
            #[cfg(feature = "canvas")]
            Self::SwapChainCanvas(_) => ReconcileKind::Native(NativeKind::SwapChainCanvas),
            #[cfg(feature = "canvas")]
            Self::SwapChainHost(_) => ReconcileKind::Native(NativeKind::SwapChainHost),
            #[cfg(feature = "webview")]
            Self::WebViewHost(_) => ReconcileKind::Native(NativeKind::WebViewHost),
            Self::Image { .. } => ReconcileKind::Native(NativeKind::Image),
            Self::Icon(icon) => ReconcileKind::Native(match icon.kind() {
                IconKind::Symbol(_) => NativeKind::SymbolIcon,
                IconKind::Font { .. } => NativeKind::FontIcon,
                IconKind::Bitmap { .. } => NativeKind::BitmapIcon,
                IconKind::Image(_) => NativeKind::ImageIcon,
                IconKind::Path(_) => NativeKind::PathIcon,
            }),
            Self::NavigationView(_) => ReconcileKind::Native(NativeKind::NavigationView),
            Self::NavigationViewItem(_) => ReconcileKind::Native(NativeKind::NavigationViewItem),
            Self::NavigationSection => ReconcileKind::NavigationSection,
            Self::Shape(props) => ReconcileKind::Native(props.kind.native_kind()),
            Self::CommandSection => ReconcileKind::CommandSection,
            Self::AppBarButton(_) => ReconcileKind::Native(NativeKind::AppBarButton),
            Self::AppBarToggleButton(_) => ReconcileKind::Native(NativeKind::AppBarToggleButton),
            Self::AppBarSeparator => ReconcileKind::Native(NativeKind::AppBarSeparator),
            Self::ContentDialog(_) => ReconcileKind::Native(NativeKind::ContentDialog),
            Self::TeachingTip(_) => ReconcileKind::Native(NativeKind::TeachingTip),
            Self::TeachingTipOwner => ReconcileKind::TeachingTipOwner,
            Self::ToolTip(_) => ReconcileKind::ToolTipOwner,
            Self::AttachedChild(_) => ReconcileKind::AttachedChild,
            Self::Border(_) => ReconcileKind::Native(NativeKind::Border),
            Self::Button(_) => ReconcileKind::Native(NativeKind::Button),
            Self::ButtonEvent(_) => ReconcileKind::Native(NativeKind::Button),
            Self::ButtonFlyout(_) => ReconcileKind::ButtonFlyout,
            Self::ButtonMenuFlyout(_) => ReconcileKind::ButtonMenuFlyout,
            Self::ButtonCommandBarFlyout(_) => ReconcileKind::ButtonCommandBarFlyout,
            Self::DropDownButton(_) => ReconcileKind::Native(NativeKind::DropDownButton),
            Self::DropDownMenuFlyout(_) => ReconcileKind::DropDownMenuFlyout,
            Self::SplitButton(_) => ReconcileKind::Native(NativeKind::SplitButton),
            Self::SplitButtonEvent(_) => ReconcileKind::Native(NativeKind::SplitButton),
            Self::SplitButtonFlyout(_) => ReconcileKind::SplitButtonFlyout,
            Self::Flyout(_) => ReconcileKind::Native(NativeKind::Flyout),
            Self::MenuFlyout(_) => ReconcileKind::Native(NativeKind::MenuFlyout),
            Self::MenuBar(_) => ReconcileKind::Native(NativeKind::MenuBar),
            Self::CommandBarFlyout(_) => ReconcileKind::Native(NativeKind::CommandBarFlyout),
            Self::HyperlinkButton(_) => ReconcileKind::Native(NativeKind::HyperlinkButton),
            Self::RepeatButton(_) => ReconcileKind::Native(NativeKind::RepeatButton),
            Self::ToggleButton(_) => ReconcileKind::Native(NativeKind::ToggleButton),
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
            Self::CheckBox(_) => ReconcileKind::Native(NativeKind::CheckBox),
            Self::RadioButton(_) => ReconcileKind::Native(NativeKind::RadioButton),
            Self::TextBlock(_) => ReconcileKind::Native(NativeKind::TextBlock),
            Self::TextBox(_) => ReconcileKind::Native(NativeKind::TextBox),
            Self::PasswordBox(_) => ReconcileKind::Native(NativeKind::PasswordBox),
            Self::ListBox(_) => ReconcileKind::Native(NativeKind::ListBox),
            Self::ComboBox(_) => ReconcileKind::Native(NativeKind::ComboBox),
            Self::RadioButtons(_) => ReconcileKind::Native(NativeKind::RadioButtons),
            Self::FlipView(_) => ReconcileKind::Native(NativeKind::FlipView),
            Self::TabView(_) => ReconcileKind::Native(NativeKind::TabView),
            Self::TabViewItem(_) => ReconcileKind::Native(NativeKind::TabViewItem),
            Self::SelectorBar(_) => ReconcileKind::Native(NativeKind::SelectorBar),
            Self::SelectorBarItem(_) => ReconcileKind::Native(NativeKind::SelectorBarItem),
            Self::BreadcrumbBar(_) => ReconcileKind::Native(NativeKind::BreadcrumbBar),
            Self::AutoSuggestBox(_) => ReconcileKind::Native(NativeKind::AutoSuggestBox),
            Self::Pivot(_) => ReconcileKind::Native(NativeKind::Pivot),
            Self::PivotItem(_) => ReconcileKind::Native(NativeKind::PivotItem),
            Self::VirtualCollection(_) => ReconcileKind::VirtualCollection,
        }
    }
}

macro_rules! define_mounted_framework_prop_lookups {
    ($($(#[$attr:meta])* ($control:ident, $element_pattern:pat => $element_props:expr, $mounted_pattern:pat => $mounted_props:expr),)*) => {
        impl MountedKind {
            pub(crate) fn framework_props(&self) -> Option<&FrameworkProps> {
                match self {
                    $($(#[$attr])* $mounted_pattern => Some($mounted_props),)*
                    _ => None,
                }
            }
        }
    };
}

framework_elements!(define_mounted_framework_prop_lookups);

impl Mounted {
    pub(crate) fn new(key: Option<u64>, kind: MountedKind) -> Self {
        Self { key, kind }
    }

    pub(crate) fn needs_retirement(&self) -> bool {
        self.has_reference() || self.has_effect_hooks() || self.has_resource_hooks()
    }

    pub(crate) fn has_reference(&self) -> bool {
        matches!(self.kind, MountedKind::Reference { .. })
            || matches!(
                &self.kind,
                MountedKind::Window(window) if window.props.reference.is_some()
            )
    }

    pub(crate) fn prepare_remove(&self, _id: NodeId) {
        #[cfg(feature = "canvas")]
        match &self.kind {
            MountedKind::CanvasImage(props) => {
                if let Some(invalidator) = &props.invalidator {
                    invalidator.unbind(_id);
                }
            }
            MountedKind::SwapChainCanvas(props) => {
                if let Some(invalidator) = &props.invalidator {
                    invalidator.unbind(_id);
                }
            }
            _ => {}
        }
    }

    pub(crate) fn has_effect_hooks(&self) -> bool {
        matches!(
            &self.kind,
            MountedKind::Component { hooks, .. }
                if hooks.iter().any(|hook| hook.has_effect())
        )
    }

    pub(crate) fn has_resource_hooks(&self) -> bool {
        matches!(
            &self.kind,
            MountedKind::Component { hooks, .. }
                if hooks.iter().any(|hook| hook.has_resource())
        )
    }

    pub(crate) fn take_reference_cleanup(&mut self) -> Option<Cleanup> {
        match &self.kind {
            MountedKind::Reference { reference, .. } => reference.clear(),
            MountedKind::Window(window) => {
                if let Some(reference) = &window.props.reference {
                    reference.clear();
                }
                None
            }
            _ => None,
        }
    }
}
