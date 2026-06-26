use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CalendarView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_today_highlighted: bool,
    pub is_group_label_visible: bool,
    pub is_enabled: bool,
    pub on_selected_dates_changed: Option<Callback<()>>,
}

impl CalendarView {
    pub fn new() -> Self {
        Self {
            is_today_highlighted: true,
            is_group_label_visible: true,
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn today_highlighted(mut self, v: bool) -> Self {
        self.is_today_highlighted = v;
        self
    }

    pub fn group_label_visible(mut self, v: bool) -> Self {
        self.is_group_label_visible = v;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_selected_dates_changed(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_selected_dates_changed = Some(f.into_unit_callback());
        self
    }
}

impl Widget for CalendarView {
    widget_header!(ControlKind::CalendarView);
    fn bindings(&self) -> PropBindings {
        generated::calendar_view_bindings(self)
    }
}

pub fn calendar_view() -> CalendarView {
    CalendarView::new()
}
