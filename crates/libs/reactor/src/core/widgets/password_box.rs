use super::*;

/// W2 — `Microsoft.UI.Xaml.Controls.PasswordBox`. Single-line password
/// entry with an optional reveal button.
#[derive(Clone, Debug, PartialEq)]
pub struct PasswordBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: String,
    pub placeholder: Option<String>,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub is_password_reveal_button_enabled: bool,
    pub reveal_mode: PasswordRevealMode,
    pub on_changed: Option<Callback<String>>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PasswordRevealMode {
    #[default]
    Peek,
    Hidden,
    Visible,
}
impl Default for PasswordBox {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: String::new(),
            placeholder: None,
            header: None,
            is_enabled: true,
            is_password_reveal_button_enabled: true,
            reveal_mode: PasswordRevealMode::Peek,
            on_changed: None,
        }
    }
}
impl PasswordBox {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn value(mut self, s: impl Into<String>) -> Self {
        self.value = s.into();
        self
    }
    pub fn placeholder(mut self, s: impl Into<String>) -> Self {
        self.placeholder = Some(s.into());
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn reveal_mode(mut self, mode: PasswordRevealMode) -> Self {
        self.reveal_mode = mode;
        self
    }
    pub fn reveal_button_enabled(mut self, v: bool) -> Self {
        self.is_password_reveal_button_enabled = v;
        self
    }
    pub fn on_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for PasswordBox {
    widget_header!(ControlKind::PasswordBox, has_events);
    fn bindings(&self) -> PropBindings {
        vec![Binding::Event(
            Event::PasswordChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        )]
    }
}
