use std::collections::BTreeSet;
use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::{SelectorBarItemProps, SelectorBarProps};
use crate::element::tree::ElementKind;
use crate::element::{Element, Icon, OptionalKeyEventFn, enforce_display_only};
use crate::framework_properties::FrameworkProps;

pub struct SelectorBarItem {
    key: u64,
    text: String,
    icon: Option<Icon>,
}

pub struct SelectorBar {
    props: SelectorBarProps,
}

impl SelectorBarItem {
    pub fn new(key: u64, text: impl Into<String>) -> Self {
        Self {
            key,
            text: text.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, value: impl Into<Icon>) -> Self {
        self.icon = Some(value.into());
        self
    }

    fn into_element(self) -> Element {
        Element::new(ElementKind::SelectorBarItem(SelectorBarItemProps {
            item_key: self.key,
            text: self.text,
            icon: self.icon,
        }))
        .key(self.key)
    }
}

impl SelectorBar {
    pub fn new(
        items: impl IntoIterator<Item = SelectorBarItem>,
        on_selection_changed: impl Fn(Option<u64>) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            items,
            Some(Rc::new(on_selection_changed)),
        ))
    }

    pub fn display(items: impl IntoIterator<Item = SelectorBarItem>) -> Framework<Self> {
        Framework::new(Self::with_handler(items, None))
    }

    fn with_handler(
        items: impl IntoIterator<Item = SelectorBarItem>,
        on_selection_changed: Option<OptionalKeyEventFn>,
    ) -> Self {
        let mut keys = BTreeSet::new();
        let items = items
            .into_iter()
            .map(|item| {
                assert!(
                    keys.insert(item.key),
                    "SelectorBar item keys must be unique"
                );
                item.into_element()
            })
            .collect();
        Self {
            props: SelectorBarProps {
                items,
                selected_key: None,
                on_selection_changed,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        assert!(
            self.props.selected_key.is_none_or(|selected| {
                self.props
                    .items
                    .iter()
                    .any(|item| item.key == Some(selected))
            }),
            "SelectorBar selected key is not present"
        );
        let mut framework = framework;
        if self.props.on_selection_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::SelectorBar(Box::new(self.props)))
    }
}

impl Framework<SelectorBar> {
    pub fn selected_key(mut self, value: Option<u64>) -> Self {
        self.control.props.selected_key = value;
        self
    }
}
