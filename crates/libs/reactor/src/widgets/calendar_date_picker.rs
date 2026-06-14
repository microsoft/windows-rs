use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CalendarDatePicker {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub placeholder_text: String,
    pub is_today_highlighted: bool,
    pub is_calendar_open: bool,
    pub is_enabled: bool,
    pub on_date_changed: Option<Callback<DateTime>>,
}

impl CalendarDatePicker {
    pub fn new() -> Self {
        Self {
            is_today_highlighted: true,
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }

    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
        self
    }

    pub fn today_highlighted(mut self, v: bool) -> Self {
        self.is_today_highlighted = v;
        self
    }

    pub fn calendar_open(mut self, v: bool) -> Self {
        self.is_calendar_open = v;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_date_changed(mut self, f: impl IntoCallback<DateTime>) -> Self {
        self.on_date_changed = Some(f.into_callback());
        self
    }
}

impl Widget for CalendarDatePicker {
    widget_header!(ControlKind::CalendarDatePicker);
    fn bindings(&self) -> PropBindings {
        generated::calendar_date_picker_bindings(self)
    }
}

pub fn calendar_date_picker() -> CalendarDatePicker {
    CalendarDatePicker::new()
}
