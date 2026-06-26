use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct HyperlinkButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub navigate_uri: Option<String>,
    pub on_click: Option<Callback<()>>,
    pub is_enabled: bool,
}
impl HyperlinkButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn navigate_uri(mut self, uri: impl Into<String>) -> Self {
        self.navigate_uri = Some(uri.into());
        self
    }
    pub fn on_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_click = Some(f.into_unit_callback());
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
        let mut out = generated::hyperlink_button_bindings(self);
        if let Some(v) = &self.navigate_uri {
            out.push(Binding::Prop(
                Prop::NavigateUri,
                PropValue::Str(v.clone()),
            ));
        }
        out
    }
}
