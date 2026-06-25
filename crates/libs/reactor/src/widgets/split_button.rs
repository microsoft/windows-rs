use super::*;

/// `Microsoft.UI.Xaml.Controls.SplitButton`. A button with a primary
/// action and a secondary dropdown action.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct SplitButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: Option<String>,
    pub is_enabled: bool,
    pub on_click: Option<Callback<()>>,
}

impl SplitButton {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }

    pub fn on_click(mut self, f: impl IntoUnitCallback) -> Self {
        self.on_click = Some(f.into_unit_callback());
        self
    }
}

impl Widget for SplitButton {
    widget_header!(ControlKind::SplitButton);
    fn bindings(&self) -> PropBindings {
        generated::split_button_bindings(self)
    }
}

pub fn split_button(content: impl Into<String>) -> SplitButton {
    SplitButton::new(content)
}
