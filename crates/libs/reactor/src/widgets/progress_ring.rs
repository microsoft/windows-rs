use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressRing {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub is_indeterminate: bool,
    pub is_active: bool,
}
impl Default for ProgressRing {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            minimum: 0.0,
            maximum: 100.0,
            is_indeterminate: true,
            is_active: true,
        }
    }
}
impl ProgressRing {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            is_indeterminate: false,
            ..Default::default()
        }
    }
    pub fn indeterminate() -> Self {
        Self::default()
    }
}

impl Widget for ProgressRing {
    widget_header!(ControlKind::ProgressRing);
    fn bindings(&self) -> PropBindings {
        generated::progress_ring_bindings(self)
    }
}
