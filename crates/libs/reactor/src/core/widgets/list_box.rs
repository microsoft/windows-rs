use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ListBoxWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub selected_index: Option<i32>,
    pub is_enabled: bool,
    pub on_selection_changed: Option<Callback<i32>>,
}

impl ListBoxWidget {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.items = items.into_iter().map(Into::into).collect();
        self
    }

    pub fn selected_index(mut self, idx: i32) -> Self {
        self.selected_index = Some(idx);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
}

impl Widget for ListBoxWidget {
    widget_header!(ControlKind::ListBox);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(4);
        if !self.items.is_empty() {
            out.push(Binding::Prop(
                Prop::ListBoxItems,
                PropValue::StrList(self.items.clone()),
            ));
        }
        if let Some(idx) = self.selected_index {
            out.push(Binding::Prop(Prop::SelectedIndex, PropValue::I32(idx)));
        }
        if !self.is_enabled {
            out.push(Binding::Prop(Prop::IsEnabled, PropValue::Bool(false)));
        }
        out.push(Binding::Event(
            Event::ListBoxSelectionChanged,
            self.on_selection_changed
                .as_ref()
                .map(|cb| EventHandler::IndexChanged(cb.clone())),
        ));
        out
    }
}

pub fn list_box() -> ListBoxWidget {
    ListBoxWidget::new()
}
