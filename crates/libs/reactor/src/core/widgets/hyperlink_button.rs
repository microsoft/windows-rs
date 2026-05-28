use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct HyperlinkButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub label: String,
    pub navigate_uri: Option<String>,
    pub on_click: Option<Callback<()>>,
    pub is_enabled: bool,
}
impl HyperlinkButton {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn navigate_uri(mut self, uri: impl Into<String>) -> Self {
        self.navigate_uri = Some(uri.into());
        self
    }
    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Callback::new(move |()| f()));
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for HyperlinkButton {
    widget_header!(ControlKind::HyperlinkButton);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(4);
        out.push(Binding::Prop(
            Prop::ButtonContent,
            PropValue::Str(self.label.clone()),
        ));
        if let Some(u) = &self.navigate_uri {
            out.push(Binding::Prop(Prop::NavigateUri, PropValue::Str(u.clone())));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::Click,
            self.on_click
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
}
