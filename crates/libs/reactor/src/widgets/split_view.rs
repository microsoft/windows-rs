use super::*;

/// Display mode for a [`SplitView`].
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
pub struct SplitView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Box<Element>,
    pub pane: Box<Element>,
    pub display_mode: SplitViewDisplayMode,
    pub is_pane_open: bool,
    pub open_pane_length: f64,
    pub compact_pane_length: f64,
    pub on_pane_closed: Option<Callback<()>>,
}

impl Default for SplitView {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            content: Box::new(Element::Empty),
            pane: Box::new(Element::Empty),
            display_mode: SplitViewDisplayMode::default(),
            is_pane_open: true,
            open_pane_length: 320.0,
            compact_pane_length: 48.0,
            on_pane_closed: None,
        }
    }
}

impl SplitView {
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
        self.open_pane_length = len;
        self
    }

    pub fn compact_pane_length(mut self, len: f64) -> Self {
        self.compact_pane_length = len;
        self
    }

    pub fn on_pane_closed(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_pane_closed = Some(f.into_unit_callback());
        self
    }
}

impl Widget for SplitView {
    widget_header!(ControlKind::SplitView);
    fn bindings(&self) -> PropBindings {
        generated::split_view_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.content)
    }
    fn pane_element(&self) -> Option<&Element> {
        Some(&self.pane)
    }
}

pub fn split_view(content: impl Into<Element>) -> SplitView {
    SplitView::new(content)
}
