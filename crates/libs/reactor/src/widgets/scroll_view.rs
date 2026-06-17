use super::*;

/// Content orientation for [`ScrollView`].
/// Maps to `ScrollingContentOrientation` in WinUI.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ScrollViewContentOrientation {
    /// Content scrolls vertically.
    #[default]
    Vertical,
    /// Content scrolls horizontally.
    Horizontal,
    /// No scrolling.
    None,
    /// Content scrolls in both directions.
    Both,
}

/// `Microsoft.UI.Xaml.Controls.ScrollView`. A modern scroll container
/// (replaces the legacy `ScrollViewer`).
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollView {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollingScrollBarVisibility,
    pub content_orientation: ScrollViewContentOrientation,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            horizontal_scroll_bar_visibility: ScrollingScrollBarVisibility::Auto,
            vertical_scroll_bar_visibility: ScrollingScrollBarVisibility::Auto,
            content_orientation: ScrollViewContentOrientation::Vertical,
        }
    }
}

impl ScrollView {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }

    pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollingScrollBarVisibility) -> Self {
        self.horizontal_scroll_bar_visibility = v;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, v: ScrollingScrollBarVisibility) -> Self {
        self.vertical_scroll_bar_visibility = v;
        self
    }

    pub fn content_orientation(mut self, v: ScrollViewContentOrientation) -> Self {
        self.content_orientation = v;
        self
    }
}

impl Widget for ScrollView {
    widget_header!(ControlKind::ScrollView);
    fn bindings(&self) -> PropBindings {
        generated::scroll_view_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn scroll_view(child: impl Into<Element>) -> ScrollView {
    ScrollView::new(child)
}
