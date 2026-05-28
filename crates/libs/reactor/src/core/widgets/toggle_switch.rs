use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ToggleSwitch {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_on: bool,
    pub on_changed: Option<Callback<bool>>,
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
    pub fn on_changed(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_changed = Some(f.into_callback());
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
        let mut out = Vec::with_capacity(6);
        out.push(Binding::Prop(Prop::IsOn, PropValue::Bool(self.is_on)));
        if let Some(s) = &self.on_content {
            out.push(Binding::Prop(Prop::OnContent, PropValue::Str(s.clone())));
        }
        if let Some(s) = &self.off_content {
            out.push(Binding::Prop(Prop::OffContent, PropValue::Str(s.clone())));
        }
        if let Some(s) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(s.clone())));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::Toggled,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::CheckedChanged(cb.clone())),
        ));
        out
    }
}
