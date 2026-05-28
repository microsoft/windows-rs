use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CalendarViewWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub is_today_highlighted: Option<bool>,
    pub is_group_label_visible: Option<bool>,
    pub is_enabled: bool,
    pub on_changed: Option<Callback<()>>,
}

impl CalendarViewWidget {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn today_highlighted(mut self, v: bool) -> Self {
        self.is_today_highlighted = Some(v);
        self
    }

    pub fn group_label_visible(mut self, v: bool) -> Self {
        self.is_group_label_visible = Some(v);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_changed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_changed = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for CalendarViewWidget {
    widget_header!(ControlKind::CalendarView);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(4);
        if let Some(v) = self.is_today_highlighted {
            out.push(Binding::Prop(Prop::IsTodayHighlighted, PropValue::Bool(v)));
        }
        if let Some(v) = self.is_group_label_visible {
            out.push(Binding::Prop(Prop::IsGroupLabelVisible, PropValue::Bool(v)));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::CalendarViewSelectionChanged,
            self.on_changed
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
}

pub fn calendar_view() -> CalendarViewWidget {
    CalendarViewWidget::new()
}
