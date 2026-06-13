use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollViewer {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub horizontal_scroll_bar_visibility: ScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollBarVisibility,
}
impl Default for ScrollViewer {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            horizontal_scroll_bar_visibility: ScrollBarVisibility::Disabled,
            vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,
        }
    }
}
impl ScrollViewer {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }
}

impl Widget for ScrollViewer {
    widget_header!(ControlKind::ScrollViewer);
    fn bindings(&self) -> PropBindings {
        generated::scroll_viewer_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

impl ScrollViewer {
    pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
        self.horizontal_scroll_bar_visibility = v;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
        self.vertical_scroll_bar_visibility = v;
        self
    }
}

pub fn scroll_viewer(child: impl Into<Element>) -> ScrollViewer {
    ScrollViewer::new(child)
}
