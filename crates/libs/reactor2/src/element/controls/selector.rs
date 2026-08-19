use std::collections::BTreeSet;
use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::{
    FlipViewProps, PivotItemProps, PivotProps, TabViewItemProps, TabViewProps,
};
use crate::element::tree::ElementKind;
use crate::element::{
    Element, EventFn, KeyEventFn, KeysEventFn, OptionalIndexEventFn, enforce_display_only,
};
use crate::framework_properties::FrameworkProps;

pub struct PivotItem {
    key: u64,
    header: String,
    content: Element,
}

pub struct Pivot {
    props: PivotProps,
}

pub struct FlipViewItem {
    key: u64,
    content: Element,
}

pub struct FlipView {
    props: FlipViewProps,
}

pub struct TabViewItem {
    key: u64,
    header: String,
    content: Element,
    closable: bool,
}

pub struct TabView {
    props: TabViewProps,
}

impl FlipViewItem {
    pub fn new(key: u64, content: Element) -> Self {
        Self { key, content }
    }
}

impl FlipView {
    pub fn new(
        items: impl IntoIterator<Item = FlipViewItem>,
        on_selection_changed: impl Fn(Option<usize>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display(items: impl IntoIterator<Item = FlipViewItem>) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: impl IntoIterator<Item = FlipViewItem>,
        on_selection_changed: Option<OptionalIndexEventFn>,
    ) -> Self {
        let mut keys = BTreeSet::new();
        let items = items
            .into_iter()
            .map(|item| {
                assert!(keys.insert(item.key), "FlipView item keys must be unique");
                item.content.key(item.key)
            })
            .collect::<Vec<_>>();
        let selected_index = (!items.is_empty()).then_some(0);
        Self {
            props: FlipViewProps {
                items,
                selected_index,
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        assert!(
            self.props
                .selected_index
                .is_none_or(|index| index < self.props.items.len()),
            "FlipView selected index is out of range"
        );
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::FlipView(Box::new(self.props)))
    }
}

impl TabViewItem {
    pub fn new(key: u64, header: impl Into<String>, content: Element) -> Self {
        Self {
            key,
            header: header.into(),
            content,
            closable: true,
        }
    }

    pub fn closable(mut self, value: bool) -> Self {
        self.closable = value;
        self
    }

    fn into_element(self) -> Element {
        Element::new(ElementKind::TabViewItem {
            child: Box::new(self.content),
            props: TabViewItemProps {
                item_key: self.key,
                header: self.header,
                closable: self.closable,
            },
        })
        .key(self.key)
    }
}

impl TabView {
    pub fn new(
        items: impl IntoIterator<Item = TabViewItem>,
        on_selection_changed: impl Fn(Option<usize>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display(items: impl IntoIterator<Item = TabViewItem>) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: impl IntoIterator<Item = TabViewItem>,
        on_selection_changed: Option<OptionalIndexEventFn>,
    ) -> Self {
        let mut keys = BTreeSet::new();
        let items = items
            .into_iter()
            .map(|item| {
                assert!(keys.insert(item.key), "TabView item keys must be unique");
                item.into_element()
            })
            .collect::<Vec<_>>();
        let selected_index = (!items.is_empty()).then_some(0);
        Self {
            props: TabViewProps {
                items,
                selected_index,
                can_reorder_tabs: false,
                is_add_tab_button_visible: false,
                on_selection_changed,
                on_close_requested: None,
                on_add_tab_button_click: None,
                on_tabs_reordered: None,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        assert!(
            self.props
                .selected_index
                .is_none_or(|index| index < self.props.items.len()),
            "TabView selected index is out of range"
        );
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::TabView(Box::new(self.props)))
    }
}

impl PivotItem {
    pub fn new(key: u64, header: impl Into<String>, content: Element) -> Self {
        Self {
            key,
            header: header.into(),
            content,
        }
    }

    fn into_element(self) -> Element {
        Element::new(ElementKind::PivotItem {
            child: Box::new(self.content),
            props: PivotItemProps {
                header: self.header,
            },
        })
        .key(self.key)
    }
}

impl Pivot {
    pub fn new(
        items: impl IntoIterator<Item = PivotItem>,
        on_selection_changed: impl Fn(Option<usize>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display(items: impl IntoIterator<Item = PivotItem>) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: impl IntoIterator<Item = PivotItem>,
        on_selection_changed: Option<OptionalIndexEventFn>,
    ) -> Self {
        let mut keys = BTreeSet::new();
        let items = items
            .into_iter()
            .map(|item| {
                assert!(keys.insert(item.key), "Pivot item keys must be unique");
                item.into_element()
            })
            .collect::<Vec<_>>();
        let selected_index = (!items.is_empty()).then_some(0);
        Self {
            props: PivotProps {
                items,
                title: None,
                selected_index,
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        assert!(
            self.props
                .selected_index
                .is_none_or(|index| index < self.props.items.len()),
            "Pivot selected index is out of range"
        );
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::Pivot(Box::new(self.props)))
    }
}

impl Framework<FlipView> {
    pub fn selected_index(mut self, value: Option<usize>) -> Self {
        self.control.props.selected_index = value;
        self
    }
}

impl Framework<TabView> {
    pub fn selected_index(mut self, value: Option<usize>) -> Self {
        self.control.props.selected_index = value;
        self
    }

    pub fn is_add_tab_button_visible(mut self, value: bool) -> Self {
        self.control.props.is_add_tab_button_visible = value;
        self
    }

    pub fn on_close_requested(mut self, handler: impl Fn(u64) + 'static) -> Self {
        self.control.props.on_close_requested = Some(Rc::new(handler) as KeyEventFn);
        self
    }

    pub fn on_add_tab_button_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_add_tab_button_click = Some(Rc::new(handler) as EventFn);
        self
    }

    pub fn reorderable(mut self, handler: impl Fn(Vec<u64>) + 'static) -> Self {
        self.control.props.can_reorder_tabs = true;
        self.control.props.on_tabs_reordered = Some(Rc::new(handler) as KeysEventFn);
        self
    }
}

impl Framework<Pivot> {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.control.props.title = Some(value.into());
        self
    }

    pub fn selected_index(mut self, value: Option<usize>) -> Self {
        self.control.props.selected_index = value;
        self
    }
}
