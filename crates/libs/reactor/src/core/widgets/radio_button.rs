use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RadioButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub label: Option<String>,
    pub is_checked: bool,
    pub on_checked: Option<Callback<()>>,
    pub group_name: Option<String>,
    pub is_enabled: bool,
}
impl RadioButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn checked(mut self, v: bool) -> Self {
        self.is_checked = v;
        self
    }
    pub fn on_checked<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_checked = Some(Callback::new(move |()| f()));
        self
    }
    pub fn group(mut self, s: impl Into<String>) -> Self {
        self.group_name = Some(s.into());
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for RadioButton {
    widget_header!(ControlKind::RadioButton, has_events);
    fn bindings(&self) -> PropBindings {
        vec![Binding::Event(
            Event::RadioChecked,
            self.on_checked
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        )]
    }
}
