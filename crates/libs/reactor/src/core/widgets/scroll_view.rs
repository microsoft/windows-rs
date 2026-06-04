use super::*;

/// Scroll bar visibility for [`ScrollViewWidget`].
/// Maps to `ScrollingScrollBarVisibility` in WinUI.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ScrollViewScrollBarVisibility {
    /// Shown only when needed.
    #[default]
    Auto,
    /// Always visible.
    Visible,
    /// Hidden but scrolling is still enabled.
    Hidden,
}

/// Content orientation for [`ScrollViewWidget`].
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
pub struct ScrollViewWidget {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub horizontal_scroll_bar_visibility: ScrollViewScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollViewScrollBarVisibility,
    pub content_orientation: ScrollViewContentOrientation,
}

impl Default for ScrollViewWidget {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            horizontal_scroll_bar_visibility: ScrollViewScrollBarVisibility::Auto,
            vertical_scroll_bar_visibility: ScrollViewScrollBarVisibility::Auto,
            content_orientation: ScrollViewContentOrientation::Vertical,
        }
    }
}

impl ScrollViewWidget {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }

    pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollViewScrollBarVisibility) -> Self {
        self.horizontal_scroll_bar_visibility = v;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, v: ScrollViewScrollBarVisibility) -> Self {
        self.vertical_scroll_bar_visibility = v;
        self
    }

    pub fn content_orientation(mut self, v: ScrollViewContentOrientation) -> Self {
        self.content_orientation = v;
        self
    }
}

impl Widget for ScrollViewWidget {
    widget_header!(ControlKind::ScrollView);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn scroll_view(child: impl Into<Element>) -> ScrollViewWidget {
    ScrollViewWidget::new(child)
}
