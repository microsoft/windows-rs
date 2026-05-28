use super::*;

/// W3 — `Microsoft.UI.Xaml.Controls.RadioButtons`. A grouped collection
/// of radio options with mutually-exclusive selection, exposed as a
/// flat list of strings.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct RadioButtons {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub header: Option<String>,
    pub items: Vec<String>,
    pub selected_index: i32,
    pub max_columns: Option<i32>,
    pub on_selection_changed: Option<Callback<i32>>,
}
impl RadioButtons {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            selected_index: -1,
            ..Default::default()
        }
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn selected_index(mut self, i: i32) -> Self {
        self.selected_index = i;
        self
    }
    pub fn max_columns(mut self, n: i32) -> Self {
        self.max_columns = Some(n);
        self
    }
    pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
}

impl Widget for RadioButtons {
    widget_header!(ControlKind::RadioButtons);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        if let Some(h) = &self.header {
            out.push(Binding::Prop(Prop::Header, PropValue::Str(h.clone())));
        }
        out.push(Binding::Prop(
            Prop::RadioButtonsItems,
            PropValue::StrList(self.items.clone()),
        ));
        out.push(Binding::Prop(
            Prop::SelectedIndex,
            PropValue::I32(self.selected_index),
        ));
        if let Some(n) = self.max_columns {
            out.push(Binding::Prop(
                Prop::RadioButtonsMaxColumns,
                PropValue::I32(n),
            ));
        }
        out.push(Binding::Event(
            Event::RadioButtonsSelectionChanged,
            self.on_selection_changed
                .as_ref()
                .map(|cb| EventHandler::IndexChanged(cb.clone())),
        ));
        out
    }
}
