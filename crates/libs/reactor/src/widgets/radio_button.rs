use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RadioButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Option<String>,
    pub is_checked: bool,
    pub on_checked: Option<Callback<()>>,
    pub group_name: String,
    pub is_enabled: bool,
}
impl RadioButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn checked(mut self, v: bool) -> Self {
        self.is_checked = v;
        self
    }
    pub fn on_checked(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_checked = Some(f.into_unit_callback());
        self
    }
    pub fn group(mut self, s: impl Into<String>) -> Self {
        self.group_name = s.into();
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for RadioButton {
    widget_header!(ControlKind::RadioButton);
    fn bindings(&self) -> PropBindings {
        generated::radio_button_bindings(self)
    }
}
