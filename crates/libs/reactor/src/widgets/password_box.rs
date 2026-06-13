use super::*;

/// W2 — `Microsoft.UI.Xaml.Controls.PasswordBox`. Single-line password
/// entry with an optional reveal button.
#[derive(Clone, Debug, PartialEq)]
pub struct PasswordBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: String,
    pub placeholder_text: String,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub is_password_reveal_button_enabled: bool,
    pub password_reveal_mode: PasswordRevealMode,
    pub on_password_changed: Option<Callback<String>>,
}
impl Default for PasswordBox {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: String::new(),
            placeholder_text: String::new(),
            header: None,
            is_enabled: true,
            is_password_reveal_button_enabled: true,
            password_reveal_mode: PasswordRevealMode::Peek,
            on_password_changed: None,
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
    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn password_reveal_mode(mut self, mode: PasswordRevealMode) -> Self {
        self.password_reveal_mode = mode;
        self
    }
    pub fn reveal_button_enabled(mut self, v: bool) -> Self {
        self.is_password_reveal_button_enabled = v;
        self
    }
    pub fn on_password_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_password_changed = Some(f.into_callback());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for PasswordBox {
    widget_header!(ControlKind::PasswordBox);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::password_box_bindings(self);
        out.push(Binding::Prop(
            Prop::Value,
            PropValue::Str(self.value.clone()),
        ));
        out
    }
}
