use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StackPanel {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub orientation: Orientation,
    pub spacing: f64,
    pub children: Vec<Element>,
}
impl StackPanel {
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
            ..Self::default()
        }
    }
    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            ..Self::default()
        }
    }
}

impl Widget for StackPanel {
    widget_header!(ControlKind::StackPanel);
    fn bindings(&self) -> PropBindings {
        generated::stack_panel_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}

impl StackPanel {
    pub fn spacing(mut self, v: f64) -> Self {
        self.spacing = v;
        self
    }
}

pub fn vstack(children: impl IntoElements) -> StackPanel {
    let mut s = StackPanel::vertical();
    s.children = children.into_elements();
    s
}

pub fn hstack(children: impl IntoElements) -> StackPanel {
    let mut s = StackPanel::horizontal();
    s.children = children.into_elements();
    s
}
