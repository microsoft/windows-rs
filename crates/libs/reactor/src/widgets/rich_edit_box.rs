use super::*;

/// `Microsoft.UI.Xaml.Controls.RichEditBox`. A multi-line rich text
/// editor with optional header and placeholder text.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct RichEditBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub text: String,
    pub placeholder_text: String,
    pub header: Option<String>,
    pub is_read_only: bool,
    pub on_text_changed: Option<Callback<String>>,
}

impl RichEditBox {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
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

    pub fn on_text_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_text_changed = Some(f.into_callback());
        self
    }
}

impl Widget for RichEditBox {
    widget_header!(ControlKind::RichEditBox);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::rich_edit_box_bindings(self);
        out.push(Binding::Prop(Prop::Text, PropValue::Str(self.text.clone())));
        out
    }
}

pub fn rich_edit_box(text: impl Into<String>) -> RichEditBox {
    RichEditBox::new(text)
}
