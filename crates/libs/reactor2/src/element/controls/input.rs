use std::rc::Rc;

use crate::element::construction::text_block;
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::*;
use crate::element::{
    BoolEventFn, Element, Framework, IntoBrushOption, TextEventFn, enforce_display_only,
    validate_border_thickness, validate_padding,
};
use crate::framework_properties::FrameworkProps;
use crate::references::{ElementRef, NativeElementRef};
pub struct CheckBox {
    label: String,
    props: CheckBoxProps,
}

impl CheckBox {
    pub fn new(
        label: impl Into<String>,
        checked: bool,
        on_toggle: impl Fn(bool) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            label.into(),
            checked,
            Some(Rc::new(on_toggle)),
        ))
    }

    pub fn display(label: impl Into<String>, checked: bool) -> Framework<Self> {
        Framework::new(Self::with_handler(label.into(), checked, None))
    }

    fn with_handler(label: String, checked: bool, on_toggle: Option<BoolEventFn>) -> Self {
        Self {
            label,
            props: CheckBoxProps {
                checked,
                on_toggle,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_toggle.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::CheckBox {
            child: Box::new(text_block(self.label)),
            props: self.props,
        })
    }
}

pub struct RadioButton {
    label: String,
    props: RadioButtonProps,
}

impl RadioButton {
    pub fn new(
        label: impl Into<String>,
        checked: bool,
        on_toggle: impl Fn(bool) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            label.into(),
            checked,
            Some(Rc::new(on_toggle)),
        ))
    }

    pub fn display(label: impl Into<String>, checked: bool) -> Framework<Self> {
        Framework::new(Self::with_handler(label.into(), checked, None))
    }

    fn with_handler(label: String, checked: bool, on_toggle: Option<BoolEventFn>) -> Self {
        Self {
            label,
            props: RadioButtonProps {
                checked,
                group_name: None,
                on_toggle,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_toggle.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::RadioButton {
            child: Box::new(text_block(self.label)),
            props: self.props,
        })
    }
}

pub struct TextBlock {
    text: String,
    pub(crate) padding: Option<Thickness>,
}

impl TextBlock {
    pub fn new(text: impl Into<String>) -> Framework<Self> {
        Framework::new({
            Self {
                text: text.into(),
                padding: None,
            }
        })
    }

    pub(crate) fn build_with_framework(self, framework: FrameworkProps) -> Element {
        Element::new(ElementKind::TextBlock(TextBlockProps {
            text: self.text,
            padding: self.padding,
            framework,
        }))
    }
}

pub struct TextBox {
    props: TextBoxProps,
    reference: Option<NativeElementRef>,
}

pub struct PasswordBox {
    props: PasswordBoxProps,
}

impl PasswordBox {
    pub fn new(
        password: impl Into<String>,
        on_change: impl Fn(String) + 'static,
    ) -> Framework<Self> {
        Framework::new(Self::with_handler(
            password.into(),
            Some(Rc::new(on_change)),
        ))
    }

    pub fn display(password: impl Into<String>) -> Framework<Self> {
        Framework::new(Self::with_handler(password.into(), None))
    }

    fn with_handler(password: String, on_change: Option<TextEventFn>) -> Self {
        Self {
            props: PasswordBoxProps {
                password,
                header: None,
                placeholder: None,
                reveal_mode: PasswordRevealMode::default(),
                on_change,
                framework: FrameworkProps::default(),
            },
        }
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        Element::new(ElementKind::PasswordBox(self.props))
    }
}

impl TextBox {
    pub fn new(text: impl Into<String>, on_change: impl Fn(String) + 'static) -> Framework<Self> {
        Framework::new(Self::with_handler(text.into(), Some(Rc::new(on_change))))
    }

    pub fn display(text: impl Into<String>) -> Framework<Self> {
        Framework::new(Self::with_handler(text.into(), None))
    }

    fn with_handler(text: String, on_change: Option<TextEventFn>) -> Self {
        Self {
            props: TextBoxProps {
                text,
                header: None,
                placeholder: None,
                accepts_return: false,
                background: None,
                border_brush: None,
                border_thickness: None,
                on_change,
                framework: FrameworkProps::default(),
            },
            reference: None,
        }
    }
}

impl Framework<TextBox> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = Some(value.into());
        self
    }

    pub fn multiline(mut self) -> Self {
        self.control.props.accepts_return = true;
        self
    }

    pub fn background(mut self, value: impl IntoBrushOption) -> Self {
        self.control.props.background = value.into_brush_option();
        self
    }

    pub fn border_brush(mut self, value: impl IntoBrushOption) -> Self {
        self.control.props.border_brush = value.into_brush_option();
        self
    }

    pub fn border_thickness(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.control.props.border_thickness = validate_border_thickness(value.into());
        self
    }

    pub fn reference(mut self, reference: &ElementRef<TextBox>) -> Self {
        self.control.reference = Some(reference.binding());
        self
    }
}

impl TextBox {
    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        let mut framework = framework;
        if self.props.on_change.is_none() {
            enforce_display_only(&mut framework);
        }
        self.props.framework = framework;
        let element = Element::new(ElementKind::TextBox(Box::new(self.props)));
        if let Some(reference) = self.reference {
            Element::new(ElementKind::Reference {
                reference,
                child: Box::new(element),
            })
        } else {
            element
        }
    }
}

impl Framework<RadioButton> {
    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.control.props.group_name = Some(value.into());
        self
    }
}

impl Framework<TextBlock> {
    pub fn padding(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.control.padding = validate_padding(value.into());
        self
    }
}

impl Framework<PasswordBox> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn placeholder_text(mut self, value: impl Into<String>) -> Self {
        self.control.props.placeholder = Some(value.into());
        self
    }

    pub fn password_reveal_mode(mut self, value: PasswordRevealMode) -> Self {
        self.control.props.reveal_mode = value;
        self
    }
}
