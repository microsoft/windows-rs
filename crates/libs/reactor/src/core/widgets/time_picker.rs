use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TimePicker {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub clock_identifier: Option<String>,
    pub minute_increment: Option<i32>,
    pub is_enabled: bool,
    pub on_changed: Option<Callback<windows_time::TimeSpan>>,
}

impl TimePicker {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }

    /// Clock format: `"12HourClock"` or `"24HourClock"`.
    pub fn clock_identifier(mut self, s: impl Into<String>) -> Self {
        self.clock_identifier = Some(s.into());
        self
    }

    pub fn minute_increment(mut self, v: i32) -> Self {
        self.minute_increment = Some(v);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_changed(mut self, f: impl IntoCallback<windows_time::TimeSpan>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
}

impl Widget for TimePicker {
    widget_header!(ControlKind::TimePicker);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::time_picker_bindings(self)
    }
}

pub fn time_picker() -> TimePicker {
    TimePicker::new()
}
