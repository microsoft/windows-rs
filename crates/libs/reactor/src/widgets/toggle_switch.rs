use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ToggleSwitch {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_on: bool,
    pub on_toggled: Option<Callback<bool>>,
    pub on_content: Option<String>,
    pub off_content: Option<String>,
    pub header: Option<String>,
    pub is_enabled: bool,
}
impl ToggleSwitch {
    pub fn new(is_on: bool) -> Self {
        Self {
            is_on,
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn on_toggled(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_toggled = Some(f.into_callback());
        self
    }
    pub fn on_content(mut self, s: impl Into<String>) -> Self {
        self.on_content = Some(s.into());
        self
    }
    pub fn off_content(mut self, s: impl Into<String>) -> Self {
        self.off_content = Some(s.into());
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for ToggleSwitch {
    widget_header!(ControlKind::ToggleSwitch);
    fn bindings(&self) -> PropBindings {
        generated::toggle_switch_bindings(self)
    }
}
