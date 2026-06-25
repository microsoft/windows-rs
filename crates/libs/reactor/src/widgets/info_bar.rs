use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InfoBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub message: String,
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
            title: String::new(),
            message: String::new(),
            severity: InfoBarSeverity::default(),
            is_open: false,
            is_closable: true,
            on_closed: None,
        }
    }
}
impl InfoBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_open: true,
            ..Default::default()
        }
    }
    pub fn message(mut self, s: impl Into<String>) -> Self {
        self.message = s.into();
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
    pub fn on_closed(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_closed = Some(f.into_unit_callback());
        self
    }
}

impl Widget for InfoBar {
    widget_header!(ControlKind::InfoBar);
    fn bindings(&self) -> PropBindings {
        generated::info_bar_bindings(self)
    }
}
