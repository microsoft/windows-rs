use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InfoBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: Option<String>,
    pub message: Option<String>,
    pub severity: InfoBarSeverity,
    pub is_open: bool,
    pub is_closable: bool,
    pub on_closed: Option<Callback<()>>,
}
impl Default for InfoBar {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            title: None,
            message: None,
            severity: InfoBarSeverity::default(),
            is_open: false,
            is_closable: true,
            on_closed: None,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum InfoBarSeverity {
    #[default]
    Informational,
    Success,
    Warning,
    Error,
}
impl InfoBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            is_open: true,
            ..Default::default()
        }
    }
    pub fn message(mut self, s: impl Into<String>) -> Self {
        self.message = Some(s.into());
        self
    }
    pub fn severity(mut self, s: InfoBarSeverity) -> Self {
        self.severity = s;
        self
    }
    /// Shortcut for `.severity(InfoBarSeverity::Informational)`.
    pub fn informational(self) -> Self {
        self.severity(InfoBarSeverity::Informational)
    }
    /// Shortcut for `.severity(InfoBarSeverity::Success)`.
    pub fn success(self) -> Self {
        self.severity(InfoBarSeverity::Success)
    }
    /// Shortcut for `.severity(InfoBarSeverity::Warning)`.
    pub fn warning(self) -> Self {
        self.severity(InfoBarSeverity::Warning)
    }
    /// Shortcut for `.severity(InfoBarSeverity::Error)`.
    pub fn error(self) -> Self {
        self.severity(InfoBarSeverity::Error)
    }
    pub fn is_open(mut self, v: bool) -> Self {
        self.is_open = v;
        self
    }
    /// Show or hide the built-in close button (`IInfoBar::IsClosable`).
    pub fn is_closable(mut self, v: bool) -> Self {
        self.is_closable = v;
        self
    }
    pub fn on_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_closed = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for InfoBar {
    widget_header!(ControlKind::InfoBar);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::info_bar_bindings(self)
    }
}
