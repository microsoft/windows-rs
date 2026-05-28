use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TimePickerWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub clock_identifier: Option<String>,
    pub minute_increment: Option<i32>,
    pub is_enabled: bool,
    pub on_changed: Option<Callback<windows_time::TimeSpan>>,
}

impl TimePickerWidget {
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

impl Widget for TimePickerWidget {
    widget_header!(ControlKind::TimePicker);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        if let Some(s) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(s.clone())));
        }
        if let Some(s) = &self.clock_identifier {
            out.push(Binding::Prop(
                Prop::ClockIdentifier,
                PropValue::Str(s.clone()),
            ));
        }
        if let Some(v) = self.minute_increment {
            out.push(Binding::Prop(Prop::MinuteIncrement, PropValue::I32(v)));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::TimeSelected,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::TimeChanged(cb.clone())),
        ));
        out
    }
}

pub fn time_picker() -> TimePickerWidget {
    TimePickerWidget::new()
}
