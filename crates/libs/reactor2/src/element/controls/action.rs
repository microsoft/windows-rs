use std::rc::Rc;

use super::content::CommandBarFlyout;
use super::menu::MenuFlyout;
use crate::element::construction::{hstack, text_block};
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::{ButtonEmphasis, FlyoutPlacement, Icon};
use crate::element::{BoolEventFn, Element, EventFn, Framework, enforce_display_only};
use crate::framework_properties::FrameworkProps;

pub struct Button {
    label: String,
    icon: Option<Icon>,
    flyout: Option<Element>,
    menu_flyout: Option<MenuFlyout>,
    command_bar_flyout: Option<CommandBarFlyout>,
    flyout_placement: Option<FlyoutPlacement>,
    on_flyout_opened: Option<EventFn>,
    on_flyout_closed: Option<EventFn>,
    props: ButtonProps,
}

pub struct DropDownButton {
    label: String,
    flyout: DropDownAccessory,
    props: DropDownButtonProps,
}

enum DropDownAccessory {
    Content(Element),
    Menu(MenuFlyout),
}

pub struct SplitButton {
    label: String,
    flyout: Option<Element>,
    flyout_placement: FlyoutPlacement,
    on_flyout_opened: Option<EventFn>,
    on_flyout_closed: Option<EventFn>,
    props: SplitButtonProps,
}

impl DropDownButton {
    pub fn new(label: impl Into<String>, flyout: Element) -> Framework<Self> {
        Framework::new({
            Self {
                label: label.into(),
                flyout: DropDownAccessory::Content(flyout),
                props: DropDownButtonProps {
                    on_opened: None,
                    on_closed: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub fn with_menu(label: impl Into<String>, flyout: MenuFlyout) -> Framework<Self> {
        Framework::new({
            Self {
                label: label.into(),
                flyout: DropDownAccessory::Menu(flyout),
                props: DropDownButtonProps {
                    on_opened: None,
                    on_closed: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        match self.flyout {
            DropDownAccessory::Content(flyout) => Element::new(ElementKind::DropDownButton(
                Box::new(DropDownButtonElement {
                    label: Box::new(text_block(self.label)),
                    flyout: DropDownFlyoutElement::Content(Box::new(flyout)),
                    props: self.props,
                }),
            )),
            DropDownAccessory::Menu(flyout) => Element::new(ElementKind::DropDownButton(Box::new(
                DropDownButtonElement {
                    label: Box::new(text_block(self.label)),
                    flyout: DropDownFlyoutElement::Menu({
                        let mut flyout = flyout.into_props();
                        flyout.on_opened =
                            combine_events(flyout.on_opened, self.props.on_opened.clone());
                        flyout.on_closed =
                            combine_events(flyout.on_closed, self.props.on_closed.clone());
                        flyout
                    }),
                    props: self.props,
                },
            ))),
        }
    }
}

impl Button {
    pub fn new(label: impl Into<String>) -> Framework<Self> {
        Framework::new(Self {
            label: label.into(),
            icon: None,
            flyout: None,
            menu_flyout: None,
            command_bar_flyout: None,
            flyout_placement: None,
            on_flyout_opened: None,
            on_flyout_closed: None,
            props: ButtonProps {
                on_click: None,
                emphasis: ButtonEmphasis::Standard,
                framework: FrameworkProps::default(),
            },
        })
    }
}

impl Framework<Button> {
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_click = Some(Rc::new(handler));
        self
    }

    pub fn icon(mut self, value: Icon) -> Self {
        self.control.icon = Some(value);
        self
    }

    pub fn emphasis(mut self, value: ButtonEmphasis) -> Self {
        self.control.props.emphasis = value;
        self
    }

    pub fn flyout(mut self, content: Element) -> Self {
        assert!(
            self.control.menu_flyout.is_none() && self.control.command_bar_flyout.is_none(),
            "button already has a flyout"
        );
        self.control.flyout = Some(content);
        self
    }

    pub fn menu_flyout(mut self, flyout: MenuFlyout) -> Self {
        assert!(
            self.control.flyout.is_none() && self.control.command_bar_flyout.is_none(),
            "button already has a flyout"
        );
        self.control.menu_flyout = Some(flyout);
        self
    }

    pub fn command_bar_flyout(mut self, flyout: CommandBarFlyout) -> Self {
        assert!(
            self.control.flyout.is_none() && self.control.menu_flyout.is_none(),
            "button already has a flyout"
        );
        self.control.command_bar_flyout = Some(flyout);
        self
    }

    pub fn flyout_placement(mut self, value: FlyoutPlacement) -> Self {
        self.control.flyout_placement = Some(value);
        self
    }

    pub fn on_flyout_opened(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.on_flyout_opened = Some(Rc::new(handler));
        self
    }

    pub fn on_flyout_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.on_flyout_closed = Some(Rc::new(handler));
        self
    }
}

impl Button {
    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        let label = button_content(self.label, self.icon);
        if let Some(flyout) = self.command_bar_flyout {
            let mut flyout = flyout.into_props();
            apply_flyout_overrides(
                &mut flyout.placement,
                &mut flyout.on_opened,
                &mut flyout.on_closed,
                self.flyout_placement,
                self.on_flyout_opened,
                self.on_flyout_closed,
            );
            Element::new(ElementKind::ButtonCommandBarFlyout {
                button: self.props,
                label: Box::new(label),
                flyout: Box::new(flyout),
            })
        } else if let Some(flyout) = self.menu_flyout {
            let mut flyout = flyout.into_props();
            apply_flyout_overrides(
                &mut flyout.placement,
                &mut flyout.on_opened,
                &mut flyout.on_closed,
                self.flyout_placement,
                self.on_flyout_opened,
                self.on_flyout_closed,
            );
            Element::new(ElementKind::ButtonMenuFlyout {
                button: self.props,
                label: Box::new(label),
                flyout,
            })
        } else if let Some(flyout) = self.flyout {
            Element::new(ElementKind::ButtonFlyout {
                button: self.props,
                content: Box::new(ButtonFlyoutElement {
                    label: Box::new(label),
                    flyout: Box::new(flyout),
                    flyout_props: FlyoutProps {
                        placement: self.flyout_placement.unwrap_or(FlyoutPlacement::Auto),
                        on_opened: self.on_flyout_opened,
                        on_closed: self.on_flyout_closed,
                    },
                }),
            })
        } else {
            Element::new(ElementKind::Button {
                child: Box::new(label),
                props: self.props,
            })
        }
    }
}

fn button_content(label: String, icon: Option<Icon>) -> Element {
    let Some(icon) = icon else {
        return text_block(label);
    };
    let icon = Element::new(ElementKind::Icon(icon));
    if label.is_empty() {
        icon
    } else {
        hstack(8.0, [icon, text_block(label)])
    }
}

fn apply_flyout_overrides(
    placement: &mut FlyoutPlacement,
    on_opened: &mut Option<EventFn>,
    on_closed: &mut Option<EventFn>,
    placement_override: Option<FlyoutPlacement>,
    opened_override: Option<EventFn>,
    closed_override: Option<EventFn>,
) {
    if let Some(value) = placement_override {
        *placement = value;
    }
    *on_opened = combine_events(on_opened.take(), opened_override);
    *on_closed = combine_events(on_closed.take(), closed_override);
}

fn combine_events(first: Option<EventFn>, second: Option<EventFn>) -> Option<EventFn> {
    match (first, second) {
        (Some(first), Some(second)) => Some(Rc::new(move || {
            first();
            second();
        })),
        (first, None) => first,
        (None, second) => second,
    }
}

impl SplitButton {
    pub fn new(label: impl Into<String>) -> Framework<Self> {
        Framework::new({
            Self {
                label: label.into(),
                flyout: None,
                flyout_placement: FlyoutPlacement::Auto,
                on_flyout_opened: None,
                on_flyout_closed: None,
                props: SplitButtonProps {
                    on_click: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        if let Some(flyout) = self.flyout {
            Element::new(ElementKind::SplitButtonFlyout {
                button: self.props,
                content: Box::new(ButtonFlyoutElement {
                    label: Box::new(text_block(self.label)),
                    flyout: Box::new(flyout),
                    flyout_props: FlyoutProps {
                        placement: self.flyout_placement,
                        on_opened: self.on_flyout_opened,
                        on_closed: self.on_flyout_closed,
                    },
                }),
            })
        } else {
            Element::new(ElementKind::SplitButton {
                child: Box::new(text_block(self.label)),
                props: self.props,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationUri(Box<str>);

impl NavigationUri {
    pub fn new(value: impl Into<Box<str>>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "navigation URI must not be empty");
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for NavigationUri {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NavigationUri {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Box<str>> for NavigationUri {
    fn from(value: Box<str>) -> Self {
        Self::new(value)
    }
}

pub struct HyperlinkButton {
    label: String,
    props: HyperlinkButtonProps,
}

impl HyperlinkButton {
    pub fn new(label: impl Into<String>) -> Framework<Self> {
        Framework::new({
            Self {
                label: label.into(),
                props: HyperlinkButtonProps {
                    navigate_uri: None,
                    on_click: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::HyperlinkButton {
            child: Box::new(text_block(self.label)),
            props: self.props,
        })
    }
}

pub struct RepeatButton {
    label: String,
    props: RepeatButtonProps,
}

impl RepeatButton {
    pub fn new(label: impl Into<String>) -> Framework<Self> {
        Framework::new({
            Self {
                label: label.into(),
                props: RepeatButtonProps {
                    delay: 500,
                    interval: 33,
                    on_click: None,
                    framework: FrameworkProps::default(),
                },
            }
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::RepeatButton {
            child: Box::new(text_block(self.label)),
            props: self.props,
        })
    }
}

pub struct ToggleButton {
    label: String,
    props: ToggleButtonProps,
}

impl ToggleButton {
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
            props: ToggleButtonProps {
                checked,
                on_toggle,
                on_click: None,
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
        Element::new(ElementKind::ToggleButton {
            child: Box::new(text_block(self.label)),
            props: self.props,
        })
    }
}

pub struct ToggleSwitch {
    props: ToggleSwitchProps,
}

impl ToggleSwitch {
    pub fn new(on: bool, on_toggle: impl Fn(bool) + 'static) -> Framework<Self> {
        Framework::new(Self::with_handler(on, Some(Rc::new(on_toggle))))
    }

    pub fn display(on: bool) -> Framework<Self> {
        Framework::new(Self::with_handler(on, None))
    }

    fn with_handler(on: bool, on_toggle: Option<BoolEventFn>) -> Self {
        Self {
            props: ToggleSwitchProps {
                on,
                header: None,
                on_content: None,
                off_content: None,
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
        Element::new(ElementKind::ToggleSwitch(self.props))
    }
}

impl Framework<DropDownButton> {
    pub fn on_opened(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_opened = Some(Rc::new(handler));
        self
    }

    pub fn on_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_closed = Some(Rc::new(handler));
        self
    }
}

impl Framework<SplitButton> {
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_click = Some(Rc::new(handler));
        self
    }

    pub fn flyout(mut self, content: Element) -> Self {
        self.control.flyout = Some(content);
        self
    }

    pub fn flyout_placement(mut self, value: FlyoutPlacement) -> Self {
        self.control.flyout_placement = value;
        self
    }

    pub fn on_flyout_opened(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.on_flyout_opened = Some(Rc::new(handler));
        self
    }

    pub fn on_flyout_closed(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.on_flyout_closed = Some(Rc::new(handler));
        self
    }
}

impl Framework<HyperlinkButton> {
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_click = Some(Rc::new(handler));
        self
    }

    pub fn navigate_uri(mut self, value: impl Into<NavigationUri>) -> Self {
        self.control.props.navigate_uri = Some(value.into().0);
        self
    }
}

impl Framework<RepeatButton> {
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_click = Some(Rc::new(handler));
        self
    }

    pub fn delay(mut self, milliseconds: i32) -> Self {
        assert!(milliseconds >= 0, "RepeatButton delay must be nonnegative");
        self.control.props.delay = milliseconds;
        self
    }

    pub fn interval(mut self, milliseconds: i32) -> Self {
        assert!(
            milliseconds >= 0,
            "RepeatButton interval must be nonnegative"
        );
        self.control.props.interval = milliseconds;
        self
    }
}

impl Framework<ToggleButton> {
    pub fn on_click(mut self, handler: impl Fn() + 'static) -> Self {
        self.control.props.on_click = Some(Rc::new(handler));
        self
    }
}

impl Framework<ToggleSwitch> {
    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.control.props.header = Some(value.into());
        self
    }

    pub fn on_content(mut self, value: impl Into<String>) -> Self {
        self.control.props.on_content = Some(value.into());
        self
    }

    pub fn off_content(mut self, value: impl Into<String>) -> Self {
        self.control.props.off_content = Some(value.into());
        self
    }
}
