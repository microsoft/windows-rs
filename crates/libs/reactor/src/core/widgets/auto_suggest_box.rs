use super::*;

/// `Microsoft.UI.Xaml.Controls.AutoSuggestBox`. A text input that displays
/// a filtered suggestion list as the user types.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct AutoSuggestBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub text: String,
    pub items: Vec<String>,
    pub placeholder_text: String,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub on_text_changed: Option<Callback<String>>,
    pub on_query_submitted: Option<Callback<String>>,
    pub on_suggestion_chosen: Option<Callback<String>>,
}

impl AutoSuggestBox {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }

    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
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

    pub fn on_text_changed<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_text_changed = Some(Callback::new(f));
        self
    }

    pub fn on_query_submitted<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_query_submitted = Some(Callback::new(f));
        self
    }

    pub fn on_suggestion_chosen<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_suggestion_chosen = Some(Callback::new(f));
        self
    }
}

impl Widget for AutoSuggestBox {
    widget_header!(ControlKind::AutoSuggestBox);
    fn bindings(&self) -> PropBindings {
        let mut out = generated_bindings::auto_suggest_box_bindings(self);
        out.push(Binding::Prop(Prop::Text, PropValue::Str(self.text.clone())));
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}

pub fn auto_suggest_box(text: impl Into<String>) -> AutoSuggestBox {
    AutoSuggestBox::new(text)
}
