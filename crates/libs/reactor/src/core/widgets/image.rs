use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Image {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub source: String,
    pub stretch: ImageStretch,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ImageStretch {
    #[default]
    Uniform,
    UniformToFill,
    Fill,
    None,
}
impl Image {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            ..Default::default()
        }
    }
    pub fn stretch(mut self, v: ImageStretch) -> Self {
        self.stretch = v;
        self
    }
}

impl Widget for Image {
    widget_header!(ControlKind::Image);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
}
