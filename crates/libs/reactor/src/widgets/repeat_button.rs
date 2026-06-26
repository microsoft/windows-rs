use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct RepeatButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub on_click: Option<Callback<()>>,
    pub is_enabled: bool,
    pub delay: i32,
    pub interval: i32,
}

impl Default for RepeatButton {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            content: String::new(),
            on_click: None,
            is_enabled: true,
            delay: 500,
            interval: 33,
        }
    }
}

impl RepeatButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn on_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_click = Some(f.into_unit_callback());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    /// Initial delay in milliseconds before repeating begins (default 500).
    pub fn delay(mut self, ms: i32) -> Self {
        self.delay = ms;
        self
    }

    /// Interval in milliseconds between repeats (default 33).
    pub fn interval(mut self, ms: i32) -> Self {
        self.interval = ms;
        self
    }
}

impl Widget for RepeatButton {
    widget_header!(ControlKind::RepeatButton);
    fn bindings(&self) -> PropBindings {
        generated::repeat_button_bindings(self)
    }
}

pub fn repeat_button(content: impl Into<String>) -> RepeatButton {
    RepeatButton::new(content)
}
