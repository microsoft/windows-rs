use std::rc::Rc;

use crate::element::Framework;
use crate::element::props::{RichEditBoxProps, RichTextBlockProps};
use crate::element::tree::ElementKind;
use crate::element::{Element, TextEventFn};
use crate::framework_properties::FrameworkProps;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RichTextRun {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
}

impl RichTextRun {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RichTextHyperlink {
    pub text: String,
    pub uri: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RichTextInline {
    Run(RichTextRun),
    LineBreak,
    Hyperlink(RichTextHyperlink),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RichTextParagraph {
    pub inlines: Rc<[RichTextInline]>,
}

impl RichTextParagraph {
    pub fn new(inlines: impl IntoIterator<Item = RichTextInline>) -> Self {
        Self {
            inlines: inlines.into_iter().collect(),
        }
    }
}

pub struct RichEditBox {
    props: RichEditBoxProps,
}

impl RichEditBox {
    pub fn new(text: impl Into<String>, handler: impl Fn(String) + 'static) -> Framework<Self> {
        Framework::new(Self::with_handler(
            text.into(),
            Some(Rc::new(handler) as TextEventFn),
        ))
    }

    pub fn display(text: impl Into<String>) -> Framework<Self> {
        Framework::new(Self::with_handler(text.into(), None))
    }

    fn with_handler(text: String, on_change: Option<TextEventFn>) -> Self {
        Self {
            props: RichEditBoxProps {
                text,
                header: None,
                placeholder: None,
                read_only: false,
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        if self.props.on_change.is_none() {
            self.props.read_only = true;
        }
        self.props.framework = framework;
        Element::new(ElementKind::RichEditBox(Box::new(self.props)))
    }
}

pub struct RichTextBlock {
    props: RichTextBlockProps,
}

impl RichTextBlock {
    pub fn new(paragraphs: impl IntoIterator<Item = RichTextParagraph>) -> Framework<Self> {
        Framework::new({
            Self {
                props: RichTextBlockProps {
                    paragraphs: paragraphs.into_iter().collect(),
                    font_size: None,
                    selectable: false,
                    wrap: false,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub fn single_paragraph(inlines: impl IntoIterator<Item = RichTextInline>) -> Framework<Self> {
        Self::new([RichTextParagraph::new(inlines)])
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::RichTextBlock(Box::new(self.props)))
    }
}

impl Framework<RichEditBox> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = Some(value.into());
        self
    }

    pub fn read_only(mut self, value: bool) -> Self {
        self.control.props.read_only = value;
        self
    }
}

impl Framework<RichTextBlock> {
    pub fn font_size(mut self, value: f64) -> Self {
        assert!(
            value.is_finite() && value >= 0.0,
            "RichTextBlock font size must be finite and nonnegative"
        );
        self.control.props.font_size = Some(value);
        self
    }

    pub fn selectable(mut self, value: bool) -> Self {
        self.control.props.selectable = value;
        self
    }

    pub fn wrap(mut self, value: bool) -> Self {
        self.control.props.wrap = value;
        self
    }
}
