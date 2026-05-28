use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CalendarDatePickerWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub placeholder_text: Option<String>,
    pub is_today_highlighted: Option<bool>,
    pub is_calendar_open: Option<bool>,
    pub is_enabled: bool,
    pub on_changed: Option<Callback<Option<windows_time::DateTime>>>,
}

impl CalendarDatePickerWidget {
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

    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = Some(s.into());
        self
    }

    pub fn today_highlighted(mut self, v: bool) -> Self {
        self.is_today_highlighted = Some(v);
        self
    }

    pub fn calendar_open(mut self, v: bool) -> Self {
        self.is_calendar_open = Some(v);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_changed(mut self, f: impl IntoCallback<Option<windows_time::DateTime>>) -> Self {
        self.on_changed = Some(f.into_callback());
        self
    }
}

impl Widget for CalendarDatePickerWidget {
    widget_header!(ControlKind::CalendarDatePicker);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(6);
        if let Some(s) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(s.clone())));
        }
        if let Some(s) = &self.placeholder_text {
            out.push(Binding::Prop(Prop::Placeholder, PropValue::Str(s.clone())));
        }
        if let Some(v) = self.is_today_highlighted {
            out.push(Binding::Prop(Prop::IsTodayHighlighted, PropValue::Bool(v)));
        }
        if let Some(v) = self.is_calendar_open {
            out.push(Binding::Prop(Prop::IsCalendarOpen, PropValue::Bool(v)));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::CalendarDateSelected,
            self.on_changed.as_ref().map(|cb| {
                EventHandler::DateTimeChanged(Callback::new({
                    let cb = cb.clone();
                    move |dt| cb.invoke(Some(dt))
                }))
            }),
        ));
        out
    }
}

pub fn calendar_date_picker() -> CalendarDatePickerWidget {
    CalendarDatePickerWidget::new()
}
