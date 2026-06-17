use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DatePicker {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub day_visible: bool,
    pub month_visible: bool,
    pub year_visible: bool,
    pub is_enabled: bool,
    pub on_selected_date_changed: Option<Callback<DateTime>>,
}

impl DatePicker {
    pub fn new() -> Self {
        Self {
            day_visible: true,
            month_visible: true,
            year_visible: true,
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }

    pub fn day_visible(mut self, v: bool) -> Self {
        self.day_visible = v;
        self
    }

    pub fn month_visible(mut self, v: bool) -> Self {
        self.month_visible = v;
        self
    }

    pub fn year_visible(mut self, v: bool) -> Self {
        self.year_visible = v;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_selected_date_changed(mut self, f: impl IntoCallback<DateTime>) -> Self {
        self.on_selected_date_changed = Some(f.into_callback());
        self
    }
}

impl Widget for DatePicker {
    widget_header!(ControlKind::DatePicker);
    fn bindings(&self) -> PropBindings {
        generated::date_picker_bindings(self)
    }
}

pub fn date_picker() -> DatePicker {
    DatePicker::new()
}
