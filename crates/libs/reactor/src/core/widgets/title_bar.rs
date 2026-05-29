use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TitleBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub title: String,
    pub subtitle: Option<String>,
    pub is_back_button_visible: bool,
    pub is_back_button_enabled: bool,
    pub is_pane_toggle_button_visible: bool,
    pub on_back_requested: Option<Callback<()>>,
    pub on_pane_toggle_requested: Option<Callback<()>>,
    pub is_tall: bool,
    /// Element placed in the TitleBar's center `Content` slot.
    pub content_element: Option<Box<Element>>,
    /// Element placed in the TitleBar's `RightHeader` slot (trailing area).
    pub footer_element: Option<Box<Element>>,
}
impl TitleBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }
    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.subtitle = Some(s.into());
        self
    }
    pub fn back_button_visible(mut self, v: bool) -> Self {
        self.is_back_button_visible = v;
        self
    }
    pub fn back_button_enabled(mut self, v: bool) -> Self {
        self.is_back_button_enabled = v;
        self
    }
    pub fn pane_toggle_button_visible(mut self, v: bool) -> Self {
        self.is_pane_toggle_button_visible = v;
        self
    }
    pub fn on_back_requested<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_back_requested = Some(Callback::new(move |()| f()));
        self
    }
    pub fn on_pane_toggle_requested<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_pane_toggle_requested = Some(Callback::new(move |()| f()));
        self
    }
    pub fn content(mut self, el: impl Into<Element>) -> Self {
        self.content_element = Some(Box::new(el.into()));
        self
    }
    pub fn footer(mut self, el: impl Into<Element>) -> Self {
        self.footer_element = Some(Box::new(el.into()));
        self
    }
    pub fn tall(mut self, v: bool) -> Self {
        self.is_tall = v;
        self
    }
}

impl Widget for TitleBar {
    widget_header!(ControlKind::TitleBar);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(7);
        out.push(Binding::Prop(
            Prop::TitleBarTitle,
            PropValue::Str(self.title.clone()),
        ));
        if let Some(s) = &self.subtitle {
            out.push(Binding::Prop(
                Prop::TitleBarSubtitle,
                PropValue::Str(s.clone()),
            ));
        }
        out.push(Binding::Prop(
            Prop::IsBackButtonVisible,
            PropValue::Bool(self.is_back_button_visible),
        ));
        out.push(Binding::Prop(
            Prop::IsBackEnabled,
            PropValue::Bool(self.is_back_button_enabled),
        ));
        out.push(Binding::Prop(
            Prop::IsPaneToggleButtonVisible,
            PropValue::Bool(self.is_pane_toggle_button_visible),
        ));
        out.push(Binding::Prop(
            Prop::TitleBarTall,
            PropValue::Bool(self.is_tall),
        ));
        out.push(Binding::Event(
            Event::TitleBarBackRequested,
            self.on_back_requested
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out.push(Binding::Event(
            Event::TitleBarPaneToggle,
            self.on_pane_toggle_requested
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
    /// Maps to WinUI `TitleBar.Content`.
    fn header_element(&self) -> Option<&Element> {
        self.content_element.as_deref()
    }
    /// Maps to WinUI `TitleBar.RightHeader`.
    fn pane_element(&self) -> Option<&Element> {
        self.footer_element.as_deref()
    }
}
