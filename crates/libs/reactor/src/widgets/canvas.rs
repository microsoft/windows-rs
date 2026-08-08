use super::*;

/// W5 - `Microsoft.UI.Xaml.Controls.Canvas`. Free-positioning panel
/// where each child is placed via the [`CanvasPosition`] attached
/// property through [`CanvasChildExt`].
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Canvas {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub children: Vec<Element>,
}
impl Canvas {
    pub fn new(children: impl IntoChildren) -> Self {
        Self {
            children: children.into_children(),
            ..Default::default()
        }
    }
}
/// Attached property for children of [`Canvas`]. Set via
/// [`CanvasChildExt::canvas_left`], [`CanvasChildExt::canvas_top`], and
/// [`CanvasChildExt::canvas_z_index`].
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
