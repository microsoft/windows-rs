use super::*;

/// Display mode for a [`SplitViewWidget`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum SplitViewDisplayMode {
    /// Pane overlays content when open, hidden when closed.
    Overlay,
    /// Pane is always visible and pushes content aside.
    #[default]
    Inline,
    /// Pane shows as a compact strip when closed, overlays content when open.
    CompactOverlay,
    /// Pane shows as a compact strip when closed, pushes content when open.
    CompactInline,
}

/// `Microsoft.UI.Xaml.Controls.SplitView`. A container with a collapsible pane
/// and a main content area.
#[derive(Clone, Debug, PartialEq)]
pub struct SplitViewWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Box<Element>,
    pub pane: Box<Element>,
    pub display_mode: SplitViewDisplayMode,
    pub is_pane_open: bool,
    pub open_pane_length: Option<f64>,
    pub compact_pane_length: Option<f64>,
    pub on_pane_closed: Option<Callback<()>>,
}

impl Default for SplitViewWidget {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            content: Box::new(Element::Empty),
            pane: Box::new(Element::Empty),
            display_mode: SplitViewDisplayMode::default(),
            is_pane_open: true,
            open_pane_length: None,
            compact_pane_length: None,
            on_pane_closed: None,
        }
    }
}

impl SplitViewWidget {
    pub fn new(content: impl Into<Element>) -> Self {
        Self {
            content: Box::new(content.into()),
            ..Default::default()
        }
    }

    pub fn pane(mut self, pane: impl Into<Element>) -> Self {
        self.pane = Box::new(pane.into());
        self
    }

    pub fn display_mode(mut self, mode: SplitViewDisplayMode) -> Self {
        self.display_mode = mode;
        self
    }

    pub fn is_pane_open(mut self, open: bool) -> Self {
        self.is_pane_open = open;
        self
    }

    pub fn open_pane_length(mut self, len: f64) -> Self {
        self.open_pane_length = Some(len);
        self
    }

    pub fn compact_pane_length(mut self, len: f64) -> Self {
        self.compact_pane_length = Some(len);
        self
    }

    pub fn on_pane_closed<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_pane_closed = Some(Callback::new(move |()| f()));
        self
    }
}

impl Widget for SplitViewWidget {
    widget_header!(ControlKind::SplitView);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(5);
        out.push(Binding::Prop(
            Prop::SplitViewDisplayMode,
            PropValue::I32(self.display_mode as i32),
        ));
        out.push(Binding::Prop(
            Prop::SplitViewIsPaneOpen,
            PropValue::Bool(self.is_pane_open),
        ));
        if let Some(len) = self.open_pane_length {
            out.push(Binding::Prop(
                Prop::SplitViewOpenPaneLength,
                PropValue::F64(len),
            ));
        }
        if let Some(len) = self.compact_pane_length {
            out.push(Binding::Prop(
                Prop::SplitViewCompactPaneLength,
                PropValue::F64(len),
            ));
        }
        out.push(Binding::Event(
            Event::SplitViewPaneClosed,
            self.on_pane_closed
                .as_ref()
                .map(|cb| EventHandler::Click(cb.clone())),
        ));
        out
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.content)
    }
    fn pane_element(&self) -> Option<&Element> {
        Some(&self.pane)
    }
}

pub fn split_view(content: impl Into<Element>) -> SplitViewWidget {
    SplitViewWidget::new(content)
}
