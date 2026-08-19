use std::rc::Rc;

use crate::element::Element;
use crate::element::Framework;
use crate::element::props::{InfoBadgeProps, InfoBarProps, PersonPictureProps};
use crate::element::tree::ElementKind;
use crate::framework_properties::FrameworkProps;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InfoBarSeverity {
    #[default]
    Informational,
    Success,
    Warning,
    Error,
}

pub struct InfoBadge {
    props: InfoBadgeProps,
}

impl InfoBadge {
    pub fn dot() -> Framework<Self> {
        Framework::new({
            Self {
                props: InfoBadgeProps {
                    value: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub fn numeric(value: i32) -> Framework<Self> {
        Framework::new({
            assert!(value >= 0, "InfoBadge value must be nonnegative");
            Self {
                props: InfoBadgeProps {
                    value: Some(value),
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::InfoBadge(self.props))
    }
}

pub struct InfoBar {
    props: InfoBarProps,
}

impl InfoBar {
    pub fn new(title: impl Into<String>) -> Framework<Self> {
        Framework::new({
            Self {
                props: InfoBarProps {
                    title: title.into(),
                    message: String::new(),
                    severity: InfoBarSeverity::Informational,
                    open: true,
                    closable: true,
                    on_close_requested: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::InfoBar(Box::new(self.props)))
    }
}

pub struct PersonPicture {
    props: PersonPictureProps,
}

impl PersonPicture {
    pub fn new() -> Framework<Self> {
        Framework::new({
            Self {
                props: PersonPictureProps {
                    display_name: None,
                    initials: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::PersonPicture(Box::new(self.props)))
    }
}

impl Default for Framework<PersonPicture> {
    fn default() -> Self {
        PersonPicture::new()
    }
}

impl Framework<InfoBar> {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.control.props.message = value.into();
        self
    }

    pub fn severity(mut self, value: InfoBarSeverity) -> Self {
        self.control.props.severity = value;
        self
    }

    pub fn informational(self) -> Self {
        self.severity(InfoBarSeverity::Informational)
    }

    pub fn success(self) -> Self {
        self.severity(InfoBarSeverity::Success)
    }

    pub fn warning(self) -> Self {
        self.severity(InfoBarSeverity::Warning)
    }

    pub fn error(self) -> Self {
        self.severity(InfoBarSeverity::Error)
    }

    pub fn open(mut self, value: bool) -> Self {
        self.control.props.open = value;
        self
    }

    pub fn closable(mut self, value: bool) -> Self {
        self.control.props.closable = value;
        self
    }

    pub fn on_close_requested(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_close_requested = Some(Rc::new(handler));
        self
    }
}

impl Framework<PersonPicture> {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.control.props.display_name = Some(value.into());
        self
    }

    pub fn initials(mut self, value: impl Into<String>) -> Self {
        self.control.props.initials = Some(value.into());
        self
    }
}
