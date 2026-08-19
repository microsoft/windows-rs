use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::{MenuBarItemSpec, MenuBarProps, MenuFlyoutProps, MenuItemSpec};
use crate::element::tree::ElementKind;
use crate::element::{Element, EventFn, FlyoutPlacement};
use crate::framework_properties::FrameworkProps;

pub struct MenuItem {
    key: u64,
    kind: MenuItemKind,
}

enum MenuItemKind {
    Item {
        text: String,
        enabled: bool,
        on_click: EventFn,
    },
    Separator,
    Submenu {
        text: String,
        items: Vec<MenuItem>,
    },
}

pub struct MenuBarItem {
    key: u64,
    title: String,
    items: Vec<MenuItem>,
}

pub struct MenuFlyout {
    items: Vec<MenuItem>,
    placement: FlyoutPlacement,
    on_opened: Option<EventFn>,
    on_closed: Option<EventFn>,
}

pub struct MenuBar {
    items: Vec<MenuBarItem>,
}

impl MenuItem {
    pub fn new(key: u64, text: impl Into<String>, handler: impl Fn() + 'static) -> Self {
        Self {
            key,
            kind: MenuItemKind::Item {
                text: text.into(),
                enabled: true,
                on_click: Rc::new(handler),
            },
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        let MenuItemKind::Item { enabled, .. } = &mut self.kind else {
            panic!("only clickable menu items can be enabled or disabled");
        };
        *enabled = value;
        self
    }

    pub fn separator(key: u64) -> Self {
        Self {
            key,
            kind: MenuItemKind::Separator,
        }
    }

    pub fn submenu(key: u64, text: impl Into<String>, items: Vec<Self>) -> Self {
        Self {
            key,
            kind: MenuItemKind::Submenu {
                text: text.into(),
                items,
            },
        }
    }
}

impl MenuBarItem {
    pub fn new(key: u64, title: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            key,
            title: title.into(),
            items,
        }
    }
}

impl MenuFlyout {
    pub fn new(items: Vec<MenuItem>) -> Self {
        Self {
            items,
            placement: FlyoutPlacement::Auto,
            on_opened: None,
            on_closed: None,
        }
    }

    pub fn placement(mut self, value: FlyoutPlacement) -> Self {
        self.placement = value;
        self
    }

    pub fn on_opened(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_opened = Some(Rc::new(handler));
        self
    }

    pub fn on_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.on_closed = Some(Rc::new(handler));
        self
    }

    pub(crate) fn into_props(self) -> MenuFlyoutProps {
        let mut keys = BTreeSet::new();
        let mut handlers = BTreeMap::new();
        let items = build_items(self.items, &mut keys, &mut handlers);
        MenuFlyoutProps {
            items,
            handlers,
            placement: self.placement,
            on_opened: self.on_opened,
            on_closed: self.on_closed,
        }
    }
}

impl MenuBar {
    pub fn new(items: Vec<MenuBarItem>) -> Framework<Self> {
        Framework::new(Self { items })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        let mut keys = BTreeSet::new();
        let mut handlers = BTreeMap::new();
        let items = self
            .items
            .into_iter()
            .map(|item| {
                assert!(keys.insert(item.key), "menu keys must be unique");
                MenuBarItemSpec {
                    key: item.key,
                    title: item.title,
                    items: build_items(item.items, &mut keys, &mut handlers),
                }
            })
            .collect();
        Element::new(ElementKind::MenuBar(MenuBarProps {
            items,
            handlers,
            framework,
        }))
    }
}

fn build_items(
    items: Vec<MenuItem>,
    keys: &mut BTreeSet<u64>,
    handlers: &mut BTreeMap<u64, EventFn>,
) -> Vec<MenuItemSpec> {
    items
        .into_iter()
        .map(|item| {
            assert!(keys.insert(item.key), "menu keys must be unique");
            match item.kind {
                MenuItemKind::Item {
                    text,
                    enabled,
                    on_click,
                } => {
                    handlers.insert(item.key, on_click);
                    MenuItemSpec::Item {
                        key: item.key,
                        text,
                        enabled,
                    }
                }
                MenuItemKind::Separator => MenuItemSpec::Separator { key: item.key },
                MenuItemKind::Submenu { text, items } => MenuItemSpec::Submenu {
                    key: item.key,
                    text,
                    items: build_items(items, keys, handlers),
                },
            }
        })
        .collect()
}
