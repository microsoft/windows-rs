use std::rc::Rc;

use crate::element::Element;
use crate::element::Framework;
use crate::element::construction::text_block;
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::*;
use crate::framework_properties::FrameworkProps;
use crate::interaction::Callback;
pub struct Viewbox {
    child: Box<Element>,
    stretch: Stretch,
}

pub struct ScrollViewer {
    child: Box<Element>,
    horizontal_scroll_bar_visibility: ScrollBarVisibility,
    vertical_scroll_bar_visibility: ScrollBarVisibility,
    on_view_changed: Option<Callback<ScrollEvent>>,
}

pub struct ScrollView {
    child: Box<Element>,
    horizontal_scroll_bar_visibility: ScrollViewBarVisibility,
    vertical_scroll_bar_visibility: ScrollViewBarVisibility,
    content_orientation: ScrollOrientation,
    on_view_changed: Option<Callback<ScrollEvent>>,
}

pub struct SplitView {
    content: Box<Element>,
    pane: Box<Element>,
    display_mode: SplitViewDisplayMode,
    is_pane_open: bool,
    open_pane_length: f64,
    compact_pane_length: f64,
    on_pane_closed: Option<Callback<()>>,
}

pub struct Expander {
    header: Box<Element>,
    content: Box<Element>,
    expanded: bool,
    on_expanded_changed: Option<Callback<bool>>,
}

pub struct TeachingTip {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) open: bool,
    pub(crate) light_dismiss: bool,
    pub(crate) action_button: Option<String>,
    pub(crate) close_button: Option<String>,
    pub(crate) on_closed: Option<Callback<()>>,
    pub(crate) on_action_button_click: Option<Callback<()>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ContentDialogResult {
    #[default]
    None,
    Primary,
    Secondary,
}

pub struct ContentDialog {
    title: Box<Element>,
    content: Box<Element>,
    props: ContentDialogProps,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CommandBarDefaultLabelPosition {
    #[default]
    Bottom,
    Right,
    Collapsed,
}

pub struct CommandBar {
    props: CommandBarProps,
}

pub struct CommandBarFlyout {
    props: CommandBarFlyoutProps,
}

pub struct Image {
    props: ImageProps,
}

pub struct CommandBarItem {
    key: u64,
    kind: CommandBarItemKind,
}

enum CommandBarItemKind {
    Button(AppBarButtonProps),
    Toggle(AppBarToggleButtonProps),
    Separator,
}

impl Viewbox {
    pub fn new(child: Element) -> Framework<Self> {
        Framework::new({
            Self {
                child: Box::new(child),
                stretch: Stretch::default(),
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::Viewbox {
            child: self.child,
            props: ViewboxProps {
                stretch: self.stretch,
                framework,
            },
        })
    }
}

impl ScrollViewer {
    pub fn new(child: Element) -> Framework<Self> {
        Framework::new({
            Self {
                child: Box::new(child),
                horizontal_scroll_bar_visibility: ScrollBarVisibility::Disabled,
                vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,
                on_view_changed: None,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::ScrollViewer {
            child: self.child,
            props: ScrollViewerProps {
                horizontal_scroll_bar_visibility: self.horizontal_scroll_bar_visibility,
                vertical_scroll_bar_visibility: self.vertical_scroll_bar_visibility,
                on_view_changed: self.on_view_changed,
                framework,
            },
        })
    }
}

impl ScrollView {
    pub fn new(child: Element) -> Framework<Self> {
        Framework::new({
            Self {
                child: Box::new(child),
                horizontal_scroll_bar_visibility: ScrollViewBarVisibility::Auto,
                vertical_scroll_bar_visibility: ScrollViewBarVisibility::Auto,
                content_orientation: ScrollOrientation::Vertical,
                on_view_changed: None,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::ScrollView {
            child: self.child,
            props: ScrollViewProps {
                horizontal_scroll_bar_visibility: self.horizontal_scroll_bar_visibility,
                vertical_scroll_bar_visibility: self.vertical_scroll_bar_visibility,
                content_orientation: self.content_orientation,
                on_view_changed: self.on_view_changed,
                framework,
            },
        })
    }
}

impl SplitView {
    pub fn new(
        content: Element,
        pane: Element,
        on_pane_closed: impl Fn() + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            content,
            pane,
            Some(Callback::new(move |()| on_pane_closed())),
        ))
    }

    pub fn display(content: Element, pane: Element) -> Framework<Self> {
        Framework::new(Self::with_handler(content, pane, None))
    }

    fn with_handler(content: Element, pane: Element, on_pane_closed: Option<Callback<()>>) -> Self {
        Self {
            content: Box::new(content),
            pane: Box::new(pane),
            display_mode: SplitViewDisplayMode::Inline,
            is_pane_open: true,
            open_pane_length: 320.0,
            compact_pane_length: 48.0,
            on_pane_closed,
        }
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::SplitView(Box::new(SplitViewElement {
            content: Box::new(Element::structural_slot(
                StructuralSlot::Content,
                *self.content,
            )),
            pane: Box::new(Element::structural_slot(StructuralSlot::Pane, *self.pane)),
            props: SplitViewProps {
                display_mode: self.display_mode,
                is_pane_open: self.is_pane_open,
                open_pane_length: self.open_pane_length,
                compact_pane_length: self.compact_pane_length,
                on_pane_closed: self.on_pane_closed,
                framework,
            },
        })))
    }
}

impl Expander {
    pub fn new(
        header: Element,
        content: Element,
        on_expanded_changed: impl Fn(bool) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            header,
            content,
            Some(Callback::new(on_expanded_changed)),
        ))
    }

    pub fn display(header: Element, content: Element) -> Framework<Self> {
        Framework::new(Self::with_handler(header, content, None))
    }

    fn with_handler(
        header: Element,
        content: Element,
        on_expanded_changed: Option<Callback<bool>>,
    ) -> Self {
        Self {
            header: Box::new(header),
            content: Box::new(content),
            expanded: false,
            on_expanded_changed,
        }
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::Expander(Box::new(ExpanderElement {
            header: Box::new(Element::structural_slot(
                StructuralSlot::Header,
                *self.header,
            )),
            content: Box::new(Element::structural_slot(
                StructuralSlot::Content,
                *self.content,
            )),
            props: ExpanderProps {
                expanded: self.expanded,
                on_expanded_changed: self.on_expanded_changed,
                framework,
            },
        })))
    }
}

impl TeachingTip {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: String::new(),
            open: false,
            light_dismiss: false,
            action_button: None,
            close_button: None,
            on_closed: None,
            on_action_button_click: None,
        }
    }

    pub fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.subtitle = value.into();
        self
    }

    pub fn open(mut self, value: bool) -> Self {
        self.open = value;
        self
    }

    pub fn light_dismiss(mut self, value: bool) -> Self {
        self.light_dismiss = value;
        self
    }

    pub fn action_button(mut self, value: impl Into<String>) -> Self {
        self.action_button = Some(value.into());
        self
    }

    pub fn close_button(mut self, value: impl Into<String>) -> Self {
        self.close_button = Some(value.into());
        self
    }

    pub fn on_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_closed = Some(Callback::new(move |()| handler()));
        self
    }

    pub fn on_action_button_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_action_button_click = Some(Callback::new(move |()| handler()));
        self
    }
}

impl ContentDialog {
    pub fn new(title: impl Into<String>, content: Element) -> Self {
        Self {
            title: Box::new(text_block(title)),
            content: Box::new(content),
            props: ContentDialogProps {
                primary_button_text: String::new(),
                secondary_button_text: String::new(),
                close_button_text: String::new(),
                primary_button_enabled: true,
                secondary_button_enabled: true,
                open: false,
                on_closed: None,
            },
        }
    }

    pub fn primary_button(mut self, value: impl Into<String>) -> Self {
        self.props.primary_button_text = value.into();
        self
    }

    pub fn secondary_button(mut self, value: impl Into<String>) -> Self {
        self.props.secondary_button_text = value.into();
        self
    }

    pub fn close_button(mut self, value: impl Into<String>) -> Self {
        self.props.close_button_text = value.into();
        self
    }

    pub fn primary_button_enabled(mut self, value: bool) -> Self {
        self.props.primary_button_enabled = value;
        self
    }

    pub fn secondary_button_enabled(mut self, value: bool) -> Self {
        self.props.secondary_button_enabled = value;
        self
    }

    pub fn open(mut self, value: bool) -> Self {
        self.props.open = value;
        self
    }

    pub fn on_closed(mut self, handler: impl Fn(ContentDialogResult) + 'static) -> Self {
        self.props.on_closed = Some(Callback::new(handler));
        self
    }

    pub fn build(self) -> Element {
        Element::new(ElementKind::ContentDialog(Box::new(ContentDialogElement {
            title: Box::new(Element::structural_slot(
                StructuralSlot::Header,
                *self.title,
            )),
            content: Box::new(Element::structural_slot(
                StructuralSlot::Content,
                *self.content,
            )),
            props: self.props,
        })))
    }
}

impl CommandBar {
    pub fn new(items: impl IntoIterator<Item = CommandBarItem>) -> Framework<Self> {
        Framework::new({
            Self {
                props: CommandBarProps {
                    primary: items.into_iter().collect(),
                    secondary: Vec::new(),
                    default_label_position: CommandBarDefaultLabelPosition::Bottom,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::CommandBar(Box::new(self.props)))
    }
}

impl CommandBarFlyout {
    pub fn new(items: impl IntoIterator<Item = CommandBarItem>) -> Self {
        Self {
            props: CommandBarFlyoutProps {
                primary: items.into_iter().collect(),
                secondary: Vec::new(),
                placement: FlyoutPlacement::Auto,
                on_opened: None,
                on_closed: None,
            },
        }
    }

    pub fn secondary_commands(mut self, items: impl IntoIterator<Item = CommandBarItem>) -> Self {
        self.props.secondary = items.into_iter().collect();
        self
    }

    pub fn placement(mut self, value: FlyoutPlacement) -> Self {
        self.props.placement = value;
        self
    }

    pub fn on_opened(mut self, handler: impl Fn() + 'static) -> Self {
        self.props.on_opened = Some(Rc::new(handler));
        self
    }

    pub fn on_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.props.on_closed = Some(Rc::new(handler));
        self
    }

    pub(crate) fn into_props(self) -> CommandBarFlyoutProps {
        self.props
    }
}

impl CommandBarItem {
    pub fn button(key: u64, label: impl Into<String>, handler: impl Fn() + 'static) -> Self {
        Self {
            key,
            kind: CommandBarItemKind::Button(AppBarButtonProps {
                label: label.into(),
                enabled: true,
                icon: None,
                on_click: Rc::new(handler),
            }),
        }
    }

    pub fn toggle(
        key: u64,
        label: impl Into<String>,
        checked: bool,
        handler: impl Fn(bool) + 'static,
    ) -> Self {
        Self {
            key,
            kind: CommandBarItemKind::Toggle(AppBarToggleButtonProps {
                label: label.into(),
                enabled: true,
                checked,
                icon: None,
                on_toggled: Rc::new(handler),
            }),
        }
    }

    pub const fn separator(key: u64) -> Self {
        Self {
            key,
            kind: CommandBarItemKind::Separator,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        match &mut self.kind {
            CommandBarItemKind::Button(props) => props.enabled = value,
            CommandBarItemKind::Toggle(props) => props.enabled = value,
            CommandBarItemKind::Separator => {}
        }
        self
    }

    pub fn icon(mut self, value: impl Into<Option<Icon>>) -> Self {
        match &mut self.kind {
            CommandBarItemKind::Button(props) => props.icon = value.into(),
            CommandBarItemKind::Toggle(props) => props.icon = value.into(),
            CommandBarItemKind::Separator => {}
        }
        self
    }

    pub(crate) fn into_element(self) -> Element {
        let kind = match self.kind {
            CommandBarItemKind::Button(props) => ElementKind::AppBarButton(props),
            CommandBarItemKind::Toggle(props) => ElementKind::AppBarToggleButton(props),
            CommandBarItemKind::Separator => ElementKind::AppBarSeparator,
        };
        Element::new(kind).key(self.key)
    }
}

impl Image {
    pub fn new(source: ImageSource) -> Framework<Self> {
        Framework::new({
            Self {
                props: ImageProps {
                    source,
                    stretch: Stretch::Uniform,
                    on_load: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::Image(self.props))
    }
}

impl Framework<Viewbox> {
    pub fn stretch(mut self, value: Stretch) -> Self {
        self.control.stretch = value;
        self
    }
}

impl Framework<ScrollViewer> {
    pub fn horizontal_scroll_bar_visibility(mut self, value: ScrollBarVisibility) -> Self {
        self.control.horizontal_scroll_bar_visibility = value;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, value: ScrollBarVisibility) -> Self {
        self.control.vertical_scroll_bar_visibility = value;
        self
    }

    pub fn on_view_changed(mut self, handler: impl Fn(ScrollEvent) + 'static) -> Self {
        self.control.on_view_changed = Some(Callback::new(handler));
        self
    }
}

impl Framework<ScrollView> {
    pub fn horizontal_scroll_bar_visibility(mut self, value: ScrollViewBarVisibility) -> Self {
        self.control.horizontal_scroll_bar_visibility = value;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, value: ScrollViewBarVisibility) -> Self {
        self.control.vertical_scroll_bar_visibility = value;
        self
    }

    pub fn content_orientation(mut self, value: ScrollOrientation) -> Self {
        self.control.content_orientation = value;
        self
    }

    pub fn on_view_changed(mut self, handler: impl Fn(ScrollEvent) + 'static) -> Self {
        self.control.on_view_changed = Some(Callback::new(handler));
        self
    }
}

impl Framework<SplitView> {
    pub fn display_mode(mut self, value: SplitViewDisplayMode) -> Self {
        self.control.display_mode = value;
        self
    }

    pub fn is_pane_open(mut self, value: bool) -> Self {
        self.control.is_pane_open = value;
        self
    }

    pub fn open_pane_length(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value >= 0.0,
            "SplitView open pane length must be finite and nonnegative"
        );
        self.control.open_pane_length = value;
        self
    }

    pub fn compact_pane_length(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value >= 0.0,
            "SplitView compact pane length must be finite and nonnegative"
        );
        self.control.compact_pane_length = value;
        self
    }
}

impl Framework<Expander> {
    pub fn expanded(mut self, value: bool) -> Self {
        self.control.expanded = value;
        self
    }
}

impl Framework<CommandBar> {
    pub fn secondary_commands(mut self, items: impl IntoIterator<Item = CommandBarItem>) -> Self {
        self.control.props.secondary = items.into_iter().collect();
        self
    }

    pub fn default_label_position(mut self, value: CommandBarDefaultLabelPosition) -> Self {
        self.control.props.default_label_position = value;
        self
    }
}

impl Framework<Image> {
    pub fn stretch(mut self, value: Stretch) -> Self {
        self.control.props.stretch = value;
        self
    }

    pub fn on_load(mut self, handler: impl Fn(windows_core::Result<()>) + 'static) -> Self {
        self.control.props.on_load = Some(Callback::new(handler));
        self
    }
}
