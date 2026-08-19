use crate::element::Framework;
use crate::element::props::BreadcrumbBarProps;
use crate::element::tree::ElementKind;
use crate::element::{Element, KeyEventFn, SelectorItem, SelectorItems};
use crate::framework_properties::FrameworkProps;
use std::rc::Rc;

pub struct BreadcrumbBar {
    props: BreadcrumbBarProps,
}

impl BreadcrumbBar {
    pub fn new<T: Into<SelectorItem>>(items: impl IntoIterator<Item = T>) -> Framework<Self> {
        Self::from_items(SelectorItems::new(items))
    }

    pub fn from_items(items: SelectorItems) -> Framework<Self> {
        Framework::new({
            Self {
                props: BreadcrumbBarProps {
                    items,
                    on_item_clicked: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::BreadcrumbBar(self.props))
    }
}

impl Framework<BreadcrumbBar> {
    pub fn on_item_clicked(mut self, handler: impl Fn(u64) + 'static) -> Self {
        self.control.props.on_item_clicked = Some(Rc::new(handler) as KeyEventFn);
        self
    }
}
