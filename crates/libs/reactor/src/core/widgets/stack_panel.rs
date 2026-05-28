use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct StackPanel {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub vertical: bool,
    pub spacing: Option<f64>,
    pub children: Vec<Element>,
}
impl StackPanel {
    pub fn vertical() -> Self {
        Self {
            vertical: true,
            ..Self::default()
        }
    }
    pub fn horizontal() -> Self {
        Self {
            vertical: false,
            ..Self::default()
        }
    }
}

impl Widget for StackPanel {
    widget_header!(ControlKind::StackPanel);
    fn bindings(&self) -> PropBindings {
        let mut out = Vec::with_capacity(2);
        out.push(Binding::Prop(
            Prop::Orientation,
            PropValue::Vertical(self.vertical),
        ));
        if let Some(sp) = self.spacing {
            out.push(Binding::Prop(Prop::Spacing, PropValue::F64(sp)));
        }
        out
    }
    fn children(&self) -> Children<'_> {
        Children::Keyed(&self.children)
    }
}

impl StackPanel {
    pub fn spacing(mut self, v: f64) -> Self {
        self.spacing = Some(v);
        self
    }
}

pub fn vstack(children: impl crate::core::into_elements::IntoElements) -> StackPanel {
    let mut s = StackPanel::vertical();
    s.children = children.into_elements();
    s
}

pub fn hstack(children: impl crate::core::into_elements::IntoElements) -> StackPanel {
    let mut s = StackPanel::horizontal();
    s.children = children.into_elements();
    s
}
