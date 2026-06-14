use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressBar {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub value: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub is_indeterminate: bool,
}
impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            value: 0.0,
            minimum: 0.0,
            maximum: 100.0,
            is_indeterminate: false,
        }
    }
}
impl ProgressBar {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
    pub fn indeterminate() -> Self {
        Self {
            is_indeterminate: true,
            ..Default::default()
        }
    }
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.minimum = min;
        self.maximum = max;
        self
    }
}

impl Widget for ProgressBar {
    widget_header!(ControlKind::ProgressBar);
    fn bindings(&self) -> PropBindings {
        generated::progress_bar_bindings(self)
    }
}
