use super::*;

/// `Microsoft.UI.Xaml.Controls.ContentDialog` — modal popup with up to
/// three buttons (primary / secondary / close). Not part of the visual
/// tree: when `is_open` flips to true the backend calls `ShowAsync` and
/// raises `on_closed(result)` once the user dismisses it.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct ContentDialog {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: Option<String>,
    pub content: Option<String>,
    pub primary_button_text: String,
    pub secondary_button_text: String,
    pub close_button_text: String,
    pub is_primary_button_enabled: bool,
    pub is_secondary_button_enabled: bool,
    pub is_open: bool,
    pub on_closed: Option<Callback<ContentDialogResult>>,
}

/// Mirrors `Microsoft.UI.Xaml.Controls.ContentDialogResult`. WinUI
/// reports `None` for both close-button clicks and system dismissals
/// (Escape, back gesture).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ContentDialogResult {
    #[default]
    None,
    Primary,
    Secondary,
}

impl ContentDialogResult {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Primary,
            2 => Self::Secondary,
            _ => Self::None,
        }
    }
}

impl ContentDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            is_primary_button_enabled: true,
            is_secondary_button_enabled: true,
            ..Default::default()
        }
    }
    pub fn content(mut self, s: impl Into<String>) -> Self {
        self.content = Some(s.into());
        self
    }
    pub fn primary_button_text(mut self, s: impl Into<String>) -> Self {
        self.primary_button_text = s.into();
        self
    }
    pub fn secondary_button_text(mut self, s: impl Into<String>) -> Self {
        self.secondary_button_text = s.into();
        self
    }
    pub fn close_button_text(mut self, s: impl Into<String>) -> Self {
        self.close_button_text = s.into();
        self
    }
    pub fn is_primary_button_enabled(mut self, v: bool) -> Self {
        self.is_primary_button_enabled = v;
        self
    }
    pub fn is_secondary_button_enabled(mut self, v: bool) -> Self {
        self.is_secondary_button_enabled = v;
        self
    }
    pub fn is_open(mut self, v: bool) -> Self {
        self.is_open = v;
        self
    }
    pub fn on_closed(mut self, f: impl IntoCallback<ContentDialogResult>) -> Self {
        self.on_closed = Some(f.into_callback());
        self
    }
}

impl Widget for ContentDialog {
    widget_header!(ControlKind::ContentDialog);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::content_dialog_bindings(self);
        // Closed event needs ContentDialogResult wrapping (i32 → enum).
        let closed_cb = self.on_closed.clone().map(|cb| {
            EventHandler::I32(Callback::new(move |i: i32| {
                cb.invoke(ContentDialogResult::from_i32(i));
            }))
        });
        out.push(Binding::Event(Event::Closed, closed_cb));
        out.push(Binding::Prop(Prop::IsOpen, PropValue::Bool(self.is_open)));
        out
    }
}
