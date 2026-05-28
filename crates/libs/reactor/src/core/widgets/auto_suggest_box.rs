use super::*;

/// `Microsoft.UI.Xaml.Controls.AutoSuggestBox`. A text input that displays
/// a filtered suggestion list as the user types.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct AutoSuggestBoxWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub text: String,
    pub items: Vec<String>,
    pub placeholder: Option<String>,
    pub header: Option<String>,
    pub is_enabled: bool,
    pub on_text_changed: Option<Callback<String>>,
    pub on_query_submitted: Option<Callback<String>>,
    pub on_suggestion_chosen: Option<Callback<String>>,
}

impl AutoSuggestBoxWidget {
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

impl Widget for AutoSuggestBoxWidget {
    widget_header!(ControlKind::AutoSuggestBox);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(7);
        out.push(Binding::Prop(
            Prop::AutoSuggestText,
            PropValue::Str(self.text.clone()),
        ));
        out.push(Binding::Prop(
            Prop::AutoSuggestItems,
            PropValue::StrList(self.items.clone()),
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
        out.push(Binding::Event(
            Event::AutoSuggestTextChanged,
            self.on_text_changed
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        ));
        out.push(Binding::Event(
            Event::AutoSuggestQuerySubmitted,
            self.on_query_submitted
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        ));
        out.push(Binding::Event(
            Event::AutoSuggestSuggestionChosen,
            self.on_suggestion_chosen
                .as_ref()
                .map(|cb| EventHandler::TextChanged(cb.clone())),
        ));
        out
    }
}

pub fn auto_suggest_box(text: impl Into<String>) -> AutoSuggestBoxWidget {
    AutoSuggestBoxWidget::new(text)
}
