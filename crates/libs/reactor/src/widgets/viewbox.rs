use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Viewbox {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub stretch: Stretch,
}

impl Default for Viewbox {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            stretch: Stretch::Uniform,
        }
    }
}

impl Viewbox {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }

    pub fn stretch(mut self, v: Stretch) -> Self {
        self.stretch = v;
        self
    }
}

impl Widget for Viewbox {
    widget_header!(ControlKind::Viewbox);
    fn bindings(&self) -> PropBindings {
        generated::viewbox_bindings(self)
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

pub fn viewbox(child: impl Into<Element>) -> Viewbox {
    Viewbox::new(child)
}
