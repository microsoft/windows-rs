use super::*;

/// W5 — `Microsoft.UI.Xaml.Controls.Canvas`. Free-positioning panel
/// where each child is placed via the [`CanvasPosition`] attached
/// property (`canvas_left` / `canvas_top` on [`Element`]).
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Canvas {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub children: Vec<Element>,
}
impl Canvas {
    pub fn new<I>(children: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Element>,
    {
        Self {
            children: children.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}
/// Attached property for children of [`Canvas`]. Set via
/// [`Element::canvas_left`] / [`Element::canvas_top`] / [`Element::canvas_z_index`].
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct CanvasPosition {
    pub left: f64,
    pub top: f64,
    pub z_index: i32,
}

impl Widget for Canvas {
    widget_header!(ControlKind::Canvas);
    fn bindings(&self) -> PropBindings {
        generated::canvas_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}
