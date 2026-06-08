use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct ListBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub selected_index: Option<i32>,
    pub is_enabled: bool,
    pub on_selection_changed: Option<Callback<i32>>,
}

impl ListBox {
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

impl Widget for ListBox {
    widget_header!(ControlKind::ListBox);
    fn bindings(&self) -> PropBindings {
        let mut out = crate::core::generated_bindings::list_box_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}

pub fn list_box() -> ListBox {
    ListBox::new()
}
