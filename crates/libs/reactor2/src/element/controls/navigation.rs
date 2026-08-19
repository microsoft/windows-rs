use std::collections::BTreeSet;
use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::{NavigationViewItemProps, NavigationViewProps};
use crate::element::tree::{ElementKind, NavigationViewElement};
use crate::element::{
    BoolEventFn, Element, Icon, NavigationDisplayMode, NavigationDisplayModeEventFn,
    NavigationPaneDisplayMode, OptionalKeyEventFn, enforce_display_only,
};
use crate::framework_properties::FrameworkProps;

pub struct NavigationItem {
    key: u64,
    label: String,
    icon: Option<Icon>,
}

pub struct NavigationView {
    props: NavigationViewProps,
    items: Vec<NavigationItem>,
    content: Element,
    footer: Option<Element>,
}

impl NavigationItem {
    pub fn new(key: u64, label: impl Into<String>) -> Self {
        Self {
            key,
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, value: Icon) -> Self {
        self.icon = Some(value);
        self
    }

    fn into_element(self) -> Element {
        Element::new(ElementKind::NavigationViewItem(NavigationViewItemProps {
            item_key: self.key,
            label: self.label,
            icon: self.icon,
        }))
        .key(self.key)
    }
}

impl NavigationView {
    pub fn new(
        items: impl IntoIterator<Item = NavigationItem>,
        content: Element,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            content,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display(
        items: impl IntoIterator<Item = NavigationItem>,
        content: Element,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(items, content, None))
    }

    fn with_handler(
        items: impl IntoIterator<Item = NavigationItem>,
        content: Element,
        on_selection_changed: Option<OptionalKeyEventFn>,
    ) -> Self {
        let items = items.into_iter().collect::<Vec<_>>();
        assert!(
            items
                .iter()
                .map(|item| item.key)
                .collect::<BTreeSet<_>>()
                .len()
                == items.len(),
            "NavigationView item keys must be unique"
        );
        Self {
            props: NavigationViewProps {
                selected_key: None,
                header: None,
                pane_title: None,
                settings_visible: true,
                pane_toggle_visible: true,
                pane_open: true,
                open_pane_length: 320.0,
                pane_display_mode: NavigationPaneDisplayMode::Auto,
                on_selection_changed,
                on_pane_open_changed: None,
                on_display_mode_changed: None,
                framework: FrameworkProps::default(),
            },
            items,
            content,
            footer: None,
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        if let Some(selected) = self.props.selected_key {
            assert!(
                self.items.iter().any(|item| item.key == selected),
                "NavigationView selected key does not exist"
            );
        }
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        if self.props.on_pane_open_changed.is_none() {
            self.props.pane_toggle_visible = false;
        }
        self.props.framework = framework;
        Element::new(ElementKind::NavigationView(Box::new(
            NavigationViewElement {
                items: self
                    .items
                    .into_iter()
                    .map(NavigationItem::into_element)
                    .collect(),
                content: Box::new(self.content),
                footer: self.footer.map(Box::new),
                props: self.props,
            },
        )))
    }
}

impl Framework<NavigationView> {
    pub fn selected_key(mut self, value: Option<u64>) -> Self {
        self.control.props.selected_key = value;
        self
    }

    pub fn on_display_mode_changed(
        mut self,
        handler: impl Fn(NavigationDisplayMode) + 'static,
    ) -> Self {
        self.control.props.on_display_mode_changed =
            Some(Rc::new(handler) as NavigationDisplayModeEventFn);
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn pane_title(mut self, value: impl Into<String>) -> Self {
        self.control.props.pane_title = Some(value.into());
        self
    }

    pub fn settings_visible(mut self, value: bool) -> Self {
        self.control.props.settings_visible = value;
        self
    }

    pub fn pane_toggle_visible(mut self, value: bool) -> Self {
        self.control.props.pane_toggle_visible = value;
        self
    }

    pub fn pane_open(mut self, value: bool, handler: impl Fn(bool) + 'static) -> Self {
        self.control.props.pane_open = value;
        self.control.props.on_pane_open_changed = Some(Rc::new(handler) as BoolEventFn);
        self
    }

    pub fn display_pane_open(mut self, value: bool) -> Self {
        self.control.props.pane_open = value;
        self.control.props.on_pane_open_changed = None;
        self
    }

    pub fn open_pane_length(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value >= 0.0,
            "NavigationView open pane length must be finite and nonnegative"
        );
        self.control.props.open_pane_length = value;
        self
    }

    pub fn pane_display_mode(mut self, value: NavigationPaneDisplayMode) -> Self {
        self.control.props.pane_display_mode = value;
        self
    }

    pub fn pane_footer(mut self, value: Element) -> Self {
        self.control.footer = Some(value);
        self
    }
}
