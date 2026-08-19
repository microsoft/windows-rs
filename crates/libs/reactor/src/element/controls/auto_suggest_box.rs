use crate::element::Framework;
use crate::element::props::AutoSuggestBoxProps;
use crate::element::tree::ElementKind;
use crate::element::{
    Element, KeyEventFn, SelectorItem, SelectorItems, TextEventFn, enforce_display_only,
};
use crate::framework_properties::FrameworkProps;
use std::rc::Rc;

pub struct AutoSuggestBox {
    props: AutoSuggestBoxProps,
}

impl AutoSuggestBox {
    pub fn new(
        text: impl Into<String>,
        on_text_changed: impl Fn(String) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            text.into(),
            Some(Rc::new(on_text_changed)),
        ))
    }

    pub fn display(text: impl Into<String>) -> Framework<Self> {
        Framework::new(Self::with_handler(text.into(), None))
    }

    fn with_handler(text: String, on_text_changed: Option<TextEventFn>) -> Self {
        Self {
            props: AutoSuggestBoxProps {
                text,
                items: SelectorItems::new(std::iter::empty::<SelectorItem>()),
                placeholder: String::new(),
                header: None,
                on_text_changed,
                on_query_submitted: None,
                on_suggestion_chosen: None,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_text_changed.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::AutoSuggestBox(self.props))
    }
}

impl Framework<AutoSuggestBox> {
    pub fn items<T: Into<SelectorItem>>(mut self, items: impl IntoIterator<Item = T>) -> Self {
        self.control.props.items = SelectorItems::new(items);
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = value.into();
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn on_query_submitted(mut self, handler: impl Fn(String) + 'static) -> Self {
        self.control.props.on_query_submitted = Some(Rc::new(handler) as TextEventFn);
        self
    }

    pub fn on_suggestion_chosen(mut self, handler: impl Fn(u64) + 'static) -> Self {
        self.control.props.on_suggestion_chosen = Some(Rc::new(handler) as KeyEventFn);
        self
    }
}
