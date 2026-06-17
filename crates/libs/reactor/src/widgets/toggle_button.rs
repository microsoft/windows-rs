use super::*;

/// `Microsoft.UI.Xaml.Controls.Primitives.ToggleButton`. A button that
/// toggles between checked and unchecked states.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct ToggleButton {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub content: String,
    pub is_checked: bool,
    pub on_checked: Option<Callback<bool>>,
    pub is_enabled: bool,
}

impl ToggleButton {
    pub fn new(content: impl Into<String>, is_checked: bool) -> Self {
        Self {
            content: content.into(),
            is_checked,
            is_enabled: true,
            ..Default::default()
        }
    }

    pub fn on_checked(mut self, f: impl IntoCallback<bool>) -> Self {
        self.on_checked = Some(f.into_callback());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

impl Widget for ToggleButton {
    widget_header!(ControlKind::ToggleButton);
    fn bindings(&self) -> PropBindings {
        generated::toggle_button_bindings(self)
    }
}

/// Creates a [`ToggleButton`] with a label and initial checked state.
pub fn toggle_button(content: impl Into<String>, is_checked: bool) -> ToggleButton {
    ToggleButton::new(content, is_checked)
}
