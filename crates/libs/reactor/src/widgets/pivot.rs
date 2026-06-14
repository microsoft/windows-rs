use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PivotItem {
    pub key: Option<String>,
    pub header: String,
    pub content: Element,
}
impl PivotItem {
    pub fn new(header: impl Into<String>, content: impl Into<Element>) -> Self {
        Self {
            key: None,
            header: header.into(),
            content: content.into(),
        }
    }
}
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Pivot {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub items: Vec<PivotItem>,
    pub selected_index: i32,
    pub on_selection_changed: Option<Callback<i32>>,
    pub title: Option<String>,
}
impl Pivot {
    pub fn new<I: IntoIterator<Item = PivotItem>>(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
            ..Default::default()
        }
    }
    pub fn selected_index(mut self, i: i32) -> Self {
        self.selected_index = i;
        self
    }
    pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
    pub fn title(mut self, s: impl Into<String>) -> Self {
        self.title = Some(s.into());
        self
    }
}

impl PivotItem {
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}

impl Widget for Pivot {
    widget_header!(ControlKind::Pivot);
    fn bindings(&self) -> PropBindings {
        generated::pivot_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::PivotItems(&self.items)
    }
}
