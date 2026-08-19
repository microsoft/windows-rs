use std::rc::Rc;

use crate::element::controls::layout::Grid;
use crate::element::props::*;
use crate::element::tree::*;
use crate::element::values::Color;
use crate::element::window::*;
use crate::element::{ColorSchemeEventFn, Element, EventFn, WindowSizeEventFn};
use crate::references::{NativeWindowRef, WindowRef};
use crate::resources::{ApplicationResource, ApplicationResources};
pub struct Application {
    windows: Vec<Element>,
    resources: ApplicationResources,
}

pub struct Window {
    title: String,
    backdrop: Option<WindowBackdrop>,
    icon: Option<WindowIcon>,
    theme: WindowTheme,
    title_bar: TitleBar,
    overlapped: WindowOverlappedPolicy,
    client_size: Option<WindowSize>,
    constraints: WindowConstraints,
    presenter: WindowPresenter,
    content: Box<Element>,
    owned_windows: Vec<Element>,
    on_close_requested: EventFn,
    on_size_changed: WindowSizeEventFn,
    on_color_scheme_changed: ColorSchemeEventFn,
    reference: Option<NativeWindowRef>,
}

impl Application {
    pub fn new(windows: impl IntoIterator<Item = Element>) -> Self {
        Self {
            windows: windows.into_iter().collect(),
            resources: ApplicationResources::default(),
        }
    }

    pub fn resources<K, V>(mut self, entries: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<ApplicationResource>,
    {
        self.resources = ApplicationResources::new(entries);
        self
    }

    pub fn build(self) -> Element {
        Element::new(ElementKind::Application {
            windows: self.windows,
            props: ApplicationProps {
                resources: self.resources,
            },
        })
    }
}

impl Window {
    pub fn new(
        title: impl Into<String>,
        content: Element,
        on_close_requested: impl Fn() + 'static,
    ) -> Self {
        Self {
            title: title.into(),
            backdrop: None,
            icon: None,
            theme: WindowTheme::System,
            title_bar: TitleBar::default(),
            overlapped: WindowOverlappedPolicy::default(),
            client_size: None,
            constraints: WindowConstraints::default(),
            presenter: WindowPresenter::Default,
            content: Box::new(content),
            owned_windows: Vec::new(),
            on_close_requested: Rc::new(on_close_requested),
            on_size_changed: Rc::new(|_| {}),
            on_color_scheme_changed: Rc::new(|_| {}),
            reference: None,
        }
    }

    pub fn client_size(mut self, width: f64, height: f64) -> Self {
        assert!(
            width.is_finite() && width > 0.0 && width <= i32::MAX as f64,
            "window client width must be finite, positive, and representable in native pixels"
        );
        assert!(
            height.is_finite() && height > 0.0 && height <= i32::MAX as f64,
            "window client height must be finite, positive, and representable in native pixels"
        );
        self.client_size = Some(WindowSize { width, height });
        self
    }

    pub fn backdrop(mut self, backdrop: WindowBackdrop) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    pub fn icon(mut self, icon: WindowIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn title_bar(mut self, title_bar: impl Into<TitleBar>) -> Self {
        let title_bar = title_bar.into();
        title_bar.validate();
        self.title_bar = title_bar;
        self.validate_options();
        self
    }

    pub fn overlapped(mut self, policy: WindowOverlappedPolicy) -> Self {
        self.overlapped = policy;
        self.validate_options();
        self
    }

    pub fn presenter(mut self, presenter: WindowPresenter) -> Self {
        self.presenter = presenter;
        self.validate_options();
        self
    }

    pub fn fullscreen(self, fullscreen: bool) -> Self {
        self.presenter(if fullscreen {
            WindowPresenter::FullScreen
        } else {
            WindowPresenter::Default
        })
    }

    pub fn client_constraints(mut self, constraints: WindowConstraints) -> Self {
        constraints.validate();
        self.constraints = constraints;
        self.validate_options();
        self
    }

    pub fn owned_windows(mut self, windows: impl IntoIterator<Item = Element>) -> Self {
        self.owned_windows = windows.into_iter().collect();
        self
    }

    pub fn reference(mut self, reference: &WindowRef) -> Self {
        self.reference = Some(reference.binding());
        self
    }

    pub fn on_size_changed(mut self, handler: impl Fn(WindowSize) + 'static) -> Self {
        self.on_size_changed = Rc::new(handler);
        self
    }

    pub fn on_color_scheme_changed(mut self, handler: impl Fn(ColorScheme) + 'static) -> Self {
        self.on_color_scheme_changed = Rc::new(handler);
        self
    }

    pub fn build(self) -> Element {
        self.validate_options();
        let (title_bar, custom, custom_title_bar) = match self.title_bar.0 {
            TitleBarKind::System(title_bar) => (title_bar, None, false),
            TitleBarKind::Custom(custom) => {
                let height = custom.height;
                (
                    SystemTitleBar {
                        extend_content: true,
                        height,
                        colors: SystemTitleBarColors {
                            background: Some(Color::argb(0, 0, 0, 0)),
                            inactive_background: Some(Color::argb(0, 0, 0, 0)),
                            button_background: Some(Color::argb(0, 0, 0, 0)),
                            button_inactive_background: Some(Color::argb(0, 0, 0, 0)),
                            ..SystemTitleBarColors::default()
                        },
                        ..SystemTitleBar::default()
                    },
                    Some(custom.build()),
                    true,
                )
            }
        };
        let title_bar_content =
            custom.unwrap_or_else(|| Grid::new(std::iter::empty::<Element>()).build());
        Element::new(ElementKind::Window(Box::new(WindowElement {
            title_bar: Box::new(title_bar_content),
            content: self.content,
            owned_windows: self.owned_windows,
            props: WindowProps {
                title: self.title,
                backdrop: self.backdrop,
                icon: self.icon,
                theme: self.theme,
                title_bar,
                overlapped: self.overlapped,
                client_size: self.client_size,
                constraints: self.constraints,
                presenter: self.presenter,
                on_close_requested: self.on_close_requested,
                on_size_changed: self.on_size_changed,
                on_color_scheme_changed: self.on_color_scheme_changed,
                reference: self.reference,
            },
            custom_title_bar,
        })))
    }

    fn validate_options(&self) {
        assert!(
            self.constraints.is_empty() || self.presenter == WindowPresenter::Default,
            "window client constraints require the default presenter"
        );
        assert!(
            self.title_bar.is_default() || self.presenter == WindowPresenter::Default,
            "custom title bars require the default presenter"
        );
        assert!(
            self.overlapped.is_default() || self.presenter == WindowPresenter::Default,
            "custom overlapped policy requires the default presenter"
        );
        self.title_bar.validate();
    }
}
