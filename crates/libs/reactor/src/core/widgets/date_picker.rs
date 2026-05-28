use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DatePickerWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub day_visible: Option<bool>,
    pub month_visible: Option<bool>,
    pub year_visible: Option<bool>,
    pub is_enabled: bool,
    pub on_changed: Option<Callback<windows_time::DateTime>>,
}

impl DatePickerWidget {
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

    pub fn day_visible(mut self, v: bool) -> Self {
        self.day_visible = Some(v);
        self
    }

    pub fn month_visible(mut self, v: bool) -> Self {
        self.month_visible = Some(v);
        self
    }

    pub fn year_visible(mut self, v: bool) -> Self {
        self.year_visible = Some(v);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_changed(mut self, f: impl IntoCallback<windows_time::DateTime>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
}

impl Widget for DatePickerWidget {
    widget_header!(ControlKind::DatePicker);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(6);
        if let Some(s) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(s.clone())));
        }
        if let Some(v) = self.day_visible {
            out.push(Binding::Prop(Prop::DayVisible, PropValue::Bool(v)));
        }
        if let Some(v) = self.month_visible {
            out.push(Binding::Prop(Prop::MonthVisible, PropValue::Bool(v)));
        }
        if let Some(v) = self.year_visible {
            out.push(Binding::Prop(Prop::YearVisible, PropValue::Bool(v)));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::DateSelected,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::DateTimeChanged(cb.clone())),
        ));
        out
    }
}

pub fn date_picker() -> DatePickerWidget {
    DatePickerWidget::new()
}
