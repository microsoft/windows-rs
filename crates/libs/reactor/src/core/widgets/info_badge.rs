use super::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct InfoBadge {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: Option<i32>,
}
impl InfoBadge {
    pub fn dot() -> Self {
        Self::default()
    }
    pub fn numeric(v: i32) -> Self {
        Self {
            value: Some(v),
            ..Default::default()
        }
    }
}

impl Widget for InfoBadge {
    widget_header!(ControlKind::InfoBadge);
    fn bindings(&self) -> PropBindings {
        crate::core::generated_bindings::info_badge_bindings(self)
    }
}
