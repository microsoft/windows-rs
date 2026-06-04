use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PersonPicture {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub display_name: Option<String>,
    pub initials: Option<String>,
}
impl PersonPicture {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn display_name(mut self, s: impl Into<String>) -> Self {
        self.display_name = Some(s.into());
        self
    }
    pub fn initials(mut self, s: impl Into<String>) -> Self {
        self.initials = Some(s.into());
        self
    }
}

impl Widget for PersonPicture {
    widget_header!(ControlKind::PersonPicture);
    fn bindings(&self) -> PropBindings {
        Vec::new()
    }
}
