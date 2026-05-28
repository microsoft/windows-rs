use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TextBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: String,
    pub on_changed: Option<Callback<String>>,
    pub placeholder: Option<String>,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub accepts_return: bool,
    pub text_wrapping_wrap: bool,
}
impl TextBox {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            is_enabled: true,
            ..Default::default()
        }
    }
}

impl Widget for TextBox {
    widget_header!(ControlKind::TextBox);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(7);
        out.push(Binding::Prop(
            Prop::TextBoxValue,
            PropValue::Str(self.value.clone()),
        ));
        if let Some(ph) = &self.placeholder {
            out.push(Binding::Prop(Prop::Placeholder, PropValue::Str(ph.clone())));
        }
        if let Some(hd) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(hd.clone())));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        if self.accepts_return {
            out.push(Binding::Prop(Prop::AcceptsReturn, PropValue::Bool(true)));
        }
        if self.text_wrapping_wrap {
            out.push(Binding::Prop(Prop::TextWrappingWrap, PropValue::Bool(true)));
        }
        out.push(Binding::Event(
            Event::TextChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        ));
        out
    }
}

impl TextBox {
    pub fn on_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_changed = Some(f.into_callback());
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

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn accepts_return(mut self, v: bool) -> Self {
        self.accepts_return = v;
        self
    }

    pub fn text_wrapping_wrap(mut self, v: bool) -> Self {
        self.text_wrapping_wrap = v;
        self
    }

    pub fn multiline(mut self) -> Self {
        self.accepts_return = true;
        self.text_wrapping_wrap = true;
        self
    }
}

pub fn text_box(value: impl Into<String>) -> TextBox {
    TextBox::new(value)
}
