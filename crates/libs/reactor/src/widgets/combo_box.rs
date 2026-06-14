use super::*;

/// W4 — `Microsoft.UI.Xaml.Controls.ComboBox`. Drop-down picker over a
/// flat list of string items.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct ComboBox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<String>,
    pub selected_index: i32,
    pub header: Option<String>,
    pub placeholder_text: String,
    pub is_editable: bool,
    pub is_enabled: bool,
    pub on_selection_changed: Option<Callback<i32>>,
}
impl ComboBox {
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            selected_index: -1,
            is_enabled: true,
            ..Default::default()
        }
    }
    pub fn selected_index(mut self, i: i32) -> Self {
        self.selected_index = i;
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn placeholder_text(mut self, s: impl Into<String>) -> Self {
        self.placeholder_text = s.into();
        self
    }
    /// Enable the text-entry mode (`IComboBox::IsEditable`).
    pub fn editable(mut self, v: bool) -> Self {
        self.is_editable = v;
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

impl Widget for ComboBox {
    widget_header!(ControlKind::ComboBox);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::combo_box_bindings(self);
        out.push(Binding::Prop(
            Prop::Items,
            PropValue::StrList(self.items.clone()),
        ));
        out
    }
}
