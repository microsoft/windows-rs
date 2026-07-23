use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct NavViewItem {
    pub content: String,
    pub tag: Option<String>,
    pub icon: Option<Icon>,
    pub is_header: bool,
    pub children: Vec<Self>,
}
impl NavViewItem {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }
    pub fn header(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            is_header: true,
            ..Default::default()
        }
    }
    pub fn tag(mut self, s: impl Into<String>) -> Self {
        self.tag = Some(s.into());
        self
    }
    pub fn icon(mut self, icon: impl Into<Icon>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    pub fn child(mut self, item: Self) -> Self {
        self.children.push(item);
        self
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct NavigationView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub menu_items: Vec<NavViewItem>,
    pub content: Box<Element>,
    pub selected_tag: Option<String>,
    pub on_selection_changed: Option<Callback<String>>,
    pub is_pane_open: bool,
    pub pane_display_mode: NavigationViewPaneDisplayMode,
    pub is_back_enabled: bool,
    pub on_back_requested: Option<Callback<()>>,
    pub is_settings_visible: bool,
    pub pane_title: String,
    pub header: Option<String>,
    pub auto_suggest_placeholder: Option<String>,
    pub auto_suggest_items: Vec<String>,
    pub on_query_submitted: Option<Callback<String>>,
    pub on_text_changed: Option<Callback<String>>,
    pub on_suggestion_chosen: Option<Callback<String>>,
    pub is_pane_toggle_button_visible: bool,
    pub is_back_button_visible: bool,
    pub open_pane_length: f64,
    pub pane_footer: Option<Box<Element>>,
}
impl Default for NavigationView {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            menu_items: Vec::new(),
            content: Box::new(Element::Empty),
            selected_tag: None,
            on_selection_changed: None,
            is_pane_open: true,
            pane_display_mode: NavigationViewPaneDisplayMode::Auto,
            is_back_enabled: false,
            on_back_requested: None,
            is_settings_visible: true,
            pane_title: String::new(),
            header: None,
            auto_suggest_placeholder: None,
            auto_suggest_items: Vec::new(),
            on_query_submitted: None,
            on_text_changed: None,
            on_suggestion_chosen: None,
            is_pane_toggle_button_visible: true,
            is_back_button_visible: true,
            open_pane_length: 320.0,
            pane_footer: None,
        }
    }
}
impl NavigationView {
    pub fn new<I: IntoIterator<Item = NavViewItem>>(
        menu_items: I,
        content: impl Into<Element>,
    ) -> Self {
        Self {
            menu_items: menu_items.into_iter().collect(),
            content: Box::new(content.into()),
            ..Default::default()
        }
    }
    pub fn selected_tag(mut self, tag: impl Into<String>) -> Self {
        self.selected_tag = Some(tag.into());
        self
    }
    pub fn on_selection_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_selection_changed = Some(f.into_callback());
        self
    }
    pub fn pane_open(mut self, v: bool) -> Self {
        self.is_pane_open = v;
        self
    }
    pub fn pane_display_mode(mut self, mode: NavigationViewPaneDisplayMode) -> Self {
        self.pane_display_mode = mode;
        self
    }
    pub fn back_enabled(mut self, v: bool) -> Self {
        self.is_back_enabled = v;
        self
    }
    pub fn on_back_requested(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_back_requested = Some(f.into_unit_callback());
        self
    }
    pub fn settings_visible(mut self, v: bool) -> Self {
        self.is_settings_visible = v;
        self
    }
    pub fn pane_title(mut self, s: impl Into<String>) -> Self {
        self.pane_title = s.into();
        self
    }
    pub fn header(mut self, s: impl Into<String>) -> Self {
        self.header = Some(s.into());
        self
    }
    pub fn auto_suggest_placeholder(mut self, s: impl Into<String>) -> Self {
        self.auto_suggest_placeholder = Some(s.into());
        self
    }
    pub fn auto_suggest_items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.auto_suggest_items = items.into_iter().map(Into::into).collect();
        self
    }
    pub fn on_query_submitted(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_query_submitted = Some(f.into_callback());
        self
    }
    pub fn on_text_changed(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_text_changed = Some(f.into_callback());
        self
    }
    pub fn on_suggestion_chosen(mut self, f: impl IntoCallback<String>) -> Self {
        self.on_suggestion_chosen = Some(f.into_callback());
        self
    }
    pub fn pane_toggle_button_visible(mut self, v: bool) -> Self {
        self.is_pane_toggle_button_visible = v;
        self
    }
    pub fn back_button_visible(mut self, v: bool) -> Self {
        self.is_back_button_visible = v;
        self
    }
    pub fn open_pane_length(mut self, len: f64) -> Self {
        self.open_pane_length = len;
        self
    }
    pub fn pane_footer(mut self, el: impl Into<Element>) -> Self {
        self.pane_footer = Some(Box::new(el.into()));
        self
    }
}

impl Widget for NavigationView {
    widget_header!(ControlKind::NavigationView);
    fn bindings(&self) -> PropBindings {
        let mut out = generated::navigation_view_bindings(self);
        out.push(Binding::Prop(
            Prop::MenuItems,
            PropValue::NavMenuItems(self.menu_items.clone()),
        ));
        if !self.is_back_button_visible {
            out.push(Binding::Prop(
                Prop::IsBackButtonVisible,
                PropValue::Bool(self.is_back_button_visible),
            ));
        }
        if let Some(v) = &self.selected_tag {
            out.push(Binding::Prop(Prop::SelectedTag, PropValue::Str(v.clone())));
        }
        if let Some(v) = &self.auto_suggest_placeholder {
            out.push(Binding::Prop(
                Prop::AutoSuggestPlaceholder,
                PropValue::Str(v.clone()),
            ));
        }
        out.push(Binding::Prop(
            Prop::AutoSuggestItems,
            PropValue::StrList(self.auto_suggest_items.clone()),
        ));
        out
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.content)
    }
    fn pane_element(&self) -> Option<&Element> {
        self.pane_footer.as_deref()
    }
}
