use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CheckBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_checked: bool,
    pub on_changed: Option<Callback<bool>>,
    pub label: Option<String>,
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
    widget_header!(ControlKind::CheckBox, has_events);
    fn bindings(&self) -> PropBindings {
        vec![Binding::Event(
            Event::CheckedChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::CheckedChanged(cb.clone())),
        )]
    }
}

impl CheckBox {
    pub fn on_changed(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }

    pub fn label(mut self, s: impl Into<String>) -> Self {
        self.label = Some(s.into());
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
