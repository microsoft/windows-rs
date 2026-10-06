use super::*;

/// Material used behind a window's content.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowBackdrop {
    #[default]
    None,
    Mica,
    MicaAlt,
    Acrylic,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTheme {
    #[default]
    System,
    Light,
    Dark,
}

/// Active light or dark color scheme for a window.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

/// Current window client size in device-independent pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowSize {
    pub width: f64,
    pub height: f64,
}

/// Restored outer-window bounds in physical screen pixels and the maximized state.
///
/// Negative positions are valid on a multi-monitor desktop. These are not client-area DIPs.
/// Applications own persistence; Reactor converts to and from Win32 workspace coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowPlacement {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub maximized: bool,
}

impl WindowPlacement {
    fn validate(self) {
        assert!(
            self.width > 0
                && self.height > 0
                && self.x.checked_add(self.width).is_some()
                && self.y.checked_add(self.height).is_some(),
            "window placement must have positive extents and representable bounds"
        );
    }
}

/// Optional window client-size limits in device-independent pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowConstraints {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
}

impl WindowConstraints {
    fn validate(self) {
        for value in [
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        ]
        .into_iter()
        .flatten()
        {
            assert!(
                value.is_finite() && value > 0.0,
                "window constraints must be finite and positive"
            );
        }
        assert!(
            self.min_width
                .zip(self.max_width)
                .is_none_or(|(min, max)| min <= max)
                && self
                    .min_height
                    .zip(self.max_height)
                    .is_none_or(|(min, max)| min <= max),
            "window minimum constraints must not exceed maximum constraints"
        );
    }
}

/// Window appearance, client sizing, and initial placement declared by a component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowVisuals {
    pub(crate) backdrop: WindowBackdrop,
    pub(crate) client_size: Option<(f64, f64)>,
    pub(crate) constraints: Option<WindowConstraints>,
    pub(crate) icon: Option<Rc<str>>,
    pub(crate) theme: WindowTheme,
    pub(crate) initial_position: Option<ScreenPoint>,
    pub(crate) initial_placement: Option<WindowPlacement>,
}

impl WindowVisuals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn backdrop(mut self, backdrop: WindowBackdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    pub fn client_size(mut self, width: f64, height: f64) -> Self {
        validate_window_size(width, height);
        self.client_size = Some((width, height));
        self
    }

    /// Sets the initial outer-window top-left position in physical screen pixels.
    ///
    /// Read only from the initial component declaration. Later changes do not move the window.
    /// Client sizing remains in DIPs. A full `initial_placement` takes precedence.
    pub fn initial_position(mut self, position: ScreenPoint) -> Self {
        self.initial_position = Some(position);
        self
    }

    /// Restores outer bounds and maximized state before the window first becomes visible.
    ///
    /// Read only from the initial component declaration. This overrides initial position and
    /// client size, but subsequent explicit changes to `client_size` still resize the window.
    /// Load saved placement before opening the window, not from a later asynchronous update.
    /// Windows adjusts fully off-screen bounds to an available monitor. Size constraints still
    /// apply. Width and height must be positive, with representable right and bottom coordinates.
    pub fn initial_placement(mut self, placement: WindowPlacement) -> Self {
        placement.validate();
        self.initial_placement = Some(placement);
        self
    }

    pub fn icon(mut self, path: impl Into<Rc<str>>) -> Self {
        let path = path.into();
        assert!(!path.is_empty(), "window icon path must not be empty");
        self.icon = Some(path);
        self
    }

    pub fn constraints(mut self, constraints: WindowConstraints) -> Self {
        constraints.validate();
        self.constraints = Some(constraints);
        self
    }

    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WindowPolicy {
    pub(crate) title: Option<String>,
    pub(crate) theme: WindowTheme,
    pub(crate) client_size: Option<(f64, f64)>,
    pub(crate) minimum_client_size: Option<(f64, f64)>,
}

impl WindowPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn client_size(mut self, width: f64, height: f64) -> Self {
        validate_window_size(width, height);
        self.client_size = Some((width, height));
        self
    }

    pub fn minimum_client_size(mut self, width: f64, height: f64) -> Self {
        validate_window_size(width, height);
        self.minimum_client_size = Some((width, height));
        self
    }
}

fn validate_window_size(width: f64, height: f64) {
    assert!(
        width.is_finite() && width > 0.0 && height.is_finite() && height > 0.0,
        "window size must be finite and positive"
    );
}
