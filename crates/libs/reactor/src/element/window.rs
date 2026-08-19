use std::rc::Rc;

use crate::element::props::TitleBarProps;
use crate::element::tree::{ElementKind, StructuralSlot, TitleBarElement};
use crate::element::values::Color;
use crate::element::{Element, EventFn, fragment};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowConstraints {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}

impl WindowConstraints {
    pub(crate) fn is_empty(self) -> bool {
        self == Self::default()
    }

    pub(crate) fn validate(self) {
        for (name, value) in [
            ("minimum client width", self.min_width),
            ("minimum client height", self.min_height),
            ("maximum client width", self.max_width),
            ("maximum client height", self.max_height),
        ] {
            assert!(
                value.is_none_or(|value| {
                    value.is_finite() && value > 0.0 && value <= f32::MAX as f64
                }),
                "{name} must be finite and positive"
            );
        }
        assert!(
            self.min_width
                .zip(self.max_width)
                .is_none_or(|(minimum, maximum)| minimum <= maximum),
            "minimum client width must not exceed maximum client width"
        );
        assert!(
            self.min_height
                .zip(self.max_height)
                .is_none_or(|(minimum, maximum)| minimum <= maximum),
            "minimum client height must not exceed maximum client height"
        );
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowPresenter {
    #[default]
    Default,
    FullScreen,
    CompactOverlay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowOverlappedPolicy {
    pub resizable: bool,
    pub minimizable: bool,
    pub maximizable: bool,
}

impl WindowOverlappedPolicy {
    pub(crate) fn is_default(self) -> bool {
        self == Self::default()
    }
}

impl Default for WindowOverlappedPolicy {
    fn default() -> Self {
        Self {
            resizable: true,
            minimizable: true,
            maximizable: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowBackdrop {
    Mica,
    MicaAlt,
    Acrylic,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowTheme {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TitleBarHeight {
    #[default]
    Standard,
    Tall,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SystemTitleBarButtonPolicy {
    #[default]
    System,
    Hidden,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SystemTitleBarIconPolicy {
    #[default]
    ShowIconAndSystemMenu,
    HideIconAndSystemMenu,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SystemTitleBarColors {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub inactive_foreground: Option<Color>,
    pub inactive_background: Option<Color>,
    pub button_foreground: Option<Color>,
    pub button_background: Option<Color>,
    pub button_hover_foreground: Option<Color>,
    pub button_hover_background: Option<Color>,
    pub button_pressed_foreground: Option<Color>,
    pub button_pressed_background: Option<Color>,
    pub button_inactive_foreground: Option<Color>,
    pub button_inactive_background: Option<Color>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SystemTitleBar {
    pub extend_content: bool,
    pub height: TitleBarHeight,
    pub buttons: SystemTitleBarButtonPolicy,
    pub icon: SystemTitleBarIconPolicy,
    pub colors: SystemTitleBarColors,
}

impl SystemTitleBar {
    pub(crate) fn is_default(self) -> bool {
        self == Self::default()
    }

    fn validate(self) {
        assert!(
            self.height == TitleBarHeight::Standard || self.extend_content,
            "tall title bars require extended window content"
        );
        assert!(
            self.buttons == SystemTitleBarButtonPolicy::System || self.extend_content,
            "hidden title-bar buttons require extended window content"
        );
        assert!(
            self.buttons == SystemTitleBarButtonPolicy::System
                || self.height == TitleBarHeight::Standard,
            "hidden title-bar buttons cannot use tall height"
        );
    }
}

pub struct TitleBar(pub(crate) TitleBarKind);

pub(crate) enum TitleBarKind {
    System(SystemTitleBar),
    Custom(CustomTitleBar),
}

impl TitleBar {
    pub fn system() -> Self {
        Self(TitleBarKind::System(SystemTitleBar::default()))
    }

    pub fn custom(title: impl Into<String>) -> Self {
        Self(TitleBarKind::Custom(CustomTitleBar {
            title: Some(title.into()),
            subtitle: None,
            content: None,
            right_header: None,
            back_button_visible: false,
            back_button_enabled: true,
            pane_toggle_button_visible: false,
            height: TitleBarHeight::Standard,
            on_back_requested: Rc::new(|| {}),
            on_pane_requested: Rc::new(|| {}),
        }))
    }

    pub fn extend_content(mut self, value: bool) -> Self {
        self.system_mut().extend_content = value;
        self
    }

    pub fn buttons(mut self, value: SystemTitleBarButtonPolicy) -> Self {
        self.system_mut().buttons = value;
        self
    }

    pub fn icon(mut self, value: SystemTitleBarIconPolicy) -> Self {
        self.system_mut().icon = value;
        self
    }

    pub fn colors(mut self, value: SystemTitleBarColors) -> Self {
        self.system_mut().colors = value;
        self
    }

    pub fn subtitle(mut self, value: impl Into<Option<String>>) -> Self {
        self.custom_mut().subtitle = value.into();
        self
    }

    pub fn content(mut self, value: impl Into<Option<Element>>) -> Self {
        self.custom_mut().content = value.into().map(Box::new);
        self
    }

    pub fn right_header(mut self, value: impl Into<Option<Element>>) -> Self {
        self.custom_mut().right_header = value.into().map(Box::new);
        self
    }

    pub fn back_button_visible(mut self, value: bool) -> Self {
        self.custom_mut().back_button_visible = value;
        self
    }

    pub fn back_button_enabled(mut self, value: bool) -> Self {
        self.custom_mut().back_button_enabled = value;
        self
    }

    pub fn pane_toggle_button_visible(mut self, value: bool) -> Self {
        self.custom_mut().pane_toggle_button_visible = value;
        self
    }

    pub fn height(mut self, value: TitleBarHeight) -> Self {
        match &mut self.0 {
            TitleBarKind::System(options) => options.height = value,
            TitleBarKind::Custom(custom) => custom.height = value,
        }
        self
    }

    pub fn on_back_requested(mut self, handler: impl Fn() + 'static) -> Self {
        self.custom_mut().on_back_requested = Rc::new(handler);
        self
    }

    pub fn on_pane_requested(mut self, handler: impl Fn() + 'static) -> Self {
        self.custom_mut().on_pane_requested = Rc::new(handler);
        self
    }

    fn custom_mut(&mut self) -> &mut CustomTitleBar {
        match &mut self.0 {
            TitleBarKind::Custom(custom) => custom,
            TitleBarKind::System(_) => panic!("custom title-bar options require TitleBar::custom"),
        }
    }

    fn system_mut(&mut self) -> &mut SystemTitleBar {
        match &mut self.0 {
            TitleBarKind::System(options) => options,
            TitleBarKind::Custom(_) => panic!("system title-bar options require TitleBar::system"),
        }
    }

    pub(crate) fn is_default(&self) -> bool {
        matches!(&self.0, TitleBarKind::System(options) if options.is_default())
    }

    pub(crate) fn validate(&self) {
        if let TitleBarKind::System(options) = &self.0 {
            options.validate();
        }
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::system()
    }
}

impl From<SystemTitleBar> for TitleBar {
    fn from(value: SystemTitleBar) -> Self {
        Self(TitleBarKind::System(value))
    }
}

pub(crate) struct CustomTitleBar {
    title: Option<String>,
    subtitle: Option<String>,
    content: Option<Box<Element>>,
    right_header: Option<Box<Element>>,
    back_button_visible: bool,
    back_button_enabled: bool,
    pane_toggle_button_visible: bool,
    pub(crate) height: TitleBarHeight,
    on_back_requested: EventFn,
    on_pane_requested: EventFn,
}

impl CustomTitleBar {
    pub(crate) fn build(self) -> Element {
        let content = self
            .content
            .map_or_else(|| fragment([]), |content| *content);
        let right_header = self
            .right_header
            .map_or_else(|| fragment([]), |right_header| *right_header);
        Element::new(ElementKind::TitleBar(Box::new(TitleBarElement {
            content: Box::new(Element::structural_slot(StructuralSlot::Content, content)),
            right_header: Box::new(Element::structural_slot(StructuralSlot::Pane, right_header)),
            props: TitleBarProps {
                title: self.title,
                subtitle: self.subtitle,
                back_button_visible: self.back_button_visible,
                back_button_enabled: self.back_button_enabled,
                pane_toggle_button_visible: self.pane_toggle_button_visible,
                height: self.height,
                on_back_requested: self.on_back_requested,
                on_pane_requested: self.on_pane_requested,
            },
        })))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowIcon {
    path: String,
}

impl WindowIcon {
    pub fn file(path: impl Into<String>) -> Self {
        let path = path.into();
        let value = std::path::Path::new(&path);
        assert!(value.is_absolute(), "window icon path must be absolute");
        assert!(
            value
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ico")),
            "window icon path must identify an .ico file"
        );
        Self { path }
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}
