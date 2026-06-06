use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct RepeatButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub on_click: Option<Callback<()>>,
    pub is_enabled: bool,
    pub delay: Option<i32>,
    pub interval: Option<i32>,
}

impl RepeatButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Callback::new(move |()| f()));
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    /// Initial delay in milliseconds before repeating begins (default 500).
    pub fn delay(mut self, ms: i32) -> Self {
        self.delay = Some(ms);
        self
    }

    /// Interval in milliseconds between repeats (default 33).
    pub fn interval(mut self, ms: i32) -> Self {
        self.interval = Some(ms);
        self
    }
}

impl Widget for RepeatButton {
    widget_header!(ControlKind::RepeatButton);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::repeat_button_bindings(self)
    }
}

pub fn repeat_button(content: impl Into<String>) -> RepeatButton {
    RepeatButton::new(content)
}
