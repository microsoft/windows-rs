use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub key: Option<String>,
    pub header: String,
    pub content: Element,
    /// When `Some`, drives `ITabViewItem::IsClosable`. Defaults to the
    /// platform default (closable) when left as `None`.
    pub is_closable: Option<bool>,
}
impl TabItem {
    pub fn new(header: impl Into<String>, content: impl Into<Element>) -> Self {
        Self {
            key: None,
            header: header.into(),
            content: content.into(),
            is_closable: None,
        }
    }
    /// Override the per-tab close button visibility (`IsClosable`).
    pub fn closable(mut self, v: bool) -> Self {
        self.is_closable = Some(v);
        self
    }
}
#[derive(Clone, Default, Debug, PartialEq)]
pub struct TabView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub tabs: Vec<TabItem>,
    pub selected_index: i32,
    pub can_reorder_tabs: bool,
    pub on_selection_changed: Option<Callback<i32>>,
    pub on_tab_close_requested: Option<Callback<String>>,
}
impl TabView {
    pub fn new<I: IntoIterator<Item = TabItem>>(tabs: I) -> Self {
        Self {
            tabs: tabs.into_iter().collect(),
            ..Default::default()
        }
    }
    pub fn selected_index(mut self, i: i32) -> Self {
        self.selected_index = i;
        self
    }
    /// Enable drag-to-reorder on tabs (`ITabView::CanReorderTabs`).
    pub fn can_reorder_tabs(mut self, v: bool) -> Self {
        self.can_reorder_tabs = v;
        self
    }
    pub fn on_selection_changed(mut self, f: impl IntoCallback<i32>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
    pub fn on_tab_close_requested<F: Fn(String) + 'static>(mut self, f: F) -> Self {
        self.on_tab_close_requested = Some(Callback::new(f));
        self
    }
}

impl TabItem {
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
}

impl Widget for TabView {
    widget_header!(ControlKind::TabView);
    fn bindings(&self) -> PropBindings {
        vec![
            Binding::Prop(Prop::SelectedIndex, PropValue::I32(self.selected_index)),
            Binding::Prop(Prop::CanReorderTabs, PropValue::Bool(self.can_reorder_tabs)),
            Binding::Event(
                Event::TabSelectionChanged,
                self.on_selection_changed
                    .as_ref()
                    .map(|cb| EventHandler::IndexChanged(cb.clone())),
            ),
            Binding::Event(
                Event::TabCloseRequested,
                self.on_tab_close_requested
                    .as_ref()
                    .map(|cb| EventHandler::TabKey(cb.clone())),
            ),
        ]
    }
    fn children(&self) -> Children<'_> {
        Children::Tabs(&self.tabs)
    }
}
