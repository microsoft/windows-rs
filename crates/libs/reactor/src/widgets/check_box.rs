use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CheckBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_checked: bool,
    pub on_checked: Option<Callback<bool>>,
    pub content: Option<String>,
    pub is_enabled: bool,
}
impl CheckBox {
    pub fn new(is_checked: bool) -> Self {
        Self {
            is_checked,
            is_enabled: true,
            ..Default::default()
        }
    }
}

impl Widget for CheckBox {
    widget_header!(ControlKind::CheckBox);
    fn bindings(&self) -> PropBindings {
        generated::check_box_bindings(self)
    }
}

impl CheckBox {
    pub fn on_checked(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_checked = Some(f.into_callback());
        self
    }

    pub fn content(mut self, s: impl Into<String>) -> Self {
        self.content = Some(s.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

pub fn check_box(is_checked: bool) -> CheckBox {
    CheckBox::new(is_checked)
}
