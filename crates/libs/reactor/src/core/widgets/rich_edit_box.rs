use super::*;

/// `Microsoft.UI.Xaml.Controls.RichEditBox`. A multi-line rich text
/// editor with optional header and placeholder text.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct RichEditBoxWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub text: String,
    pub placeholder: Option<String>,
    pub header: Option<String>,
    pub is_read_only: bool,
    pub on_changed: Option<Callback<String>>,
}

impl RichEditBoxWidget {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, s: impl Into<String>) -> Self {
        self.placeholder = Some(s.into());
        self
    }

    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }

    pub fn read_only(mut self) -> Self {
        self.is_read_only = true;
        self
    }

    pub fn on_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
}

impl Widget for RichEditBoxWidget {
    widget_header!(ControlKind::RichEditBox);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        out.push(Binding::Prop(
            Prop::RichEditBoxText,
            PropValue::Str(self.text.clone()),
        ));
        if let Some(ph) = &self.placeholder {
            out.push(Binding::Prop(Prop::Placeholder, PropValue::Str(ph.clone())));
        }
        if let Some(hd) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(hd.clone())));
        }
        if self.is_read_only {
            out.push(Binding::Prop(
                Prop::RichEditBoxIsReadOnly,
                PropValue::Bool(true),
            ));
        }
        out.push(Binding::Event(
            Event::RichEditBoxTextChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        ));
        out
    }
}

pub fn rich_edit_box(text: impl Into<String>) -> RichEditBoxWidget {
    RichEditBoxWidget::new(text)
}
